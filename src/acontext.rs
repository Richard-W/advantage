use super::*;
use num::NumCast;
use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

static NEXT_CONTEXT_ID: AtomicUsize = AtomicUsize::new(1);

lazy_static! {
    static ref CONTEXT_MAP: Mutex<HashMap<usize, Weak<dyn Any + Send + Sync>>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug)]
struct AContextInner<S: Float> {
    cid: usize,
    pub indeps: Vec<usize>,
    pub deps: Vec<usize>,
    pub ops: Vec<Operation>,
    pub vals: Vec<S>,
}

impl<S: Float> AContextInner<S> {
    /// Construct a raw AContextInner
    fn construct(cid: usize) -> Self {
        AContextInner {
            cid,
            indeps: Vec::new(),
            deps: Vec::new(),
            ops: Vec::new(),
            vals: Vec::new(),
        }
    }

    /// Create an AContextInner
    pub fn new() -> Arc<Mutex<Self>> {
        let id = NEXT_CONTEXT_ID.fetch_add(1, Ordering::SeqCst);
        // May be a problem on 32bit platforms and 64bit platform running for
        // more than a couple of millions of years. Crash and burn!
        assert!(id < usize::max_value());
        let this = Arc::new(Mutex::new(Self::construct(id)));
        let weak = Arc::downgrade(&this);
        let mut ctx_map = CONTEXT_MAP.lock().unwrap();
        ctx_map.insert(id, weak);
        this
    }

    /// Get context id
    pub fn cid(&self) -> usize {
        self.cid
    }
}

impl<S: Float> Drop for AContextInner<S> {
    fn drop(&mut self) {
        let mut ctx_map = CONTEXT_MAP.lock().unwrap();
        ctx_map.remove(&self.cid());
    }
}

/// Records a function evaluation
pub struct AContext<S: Float> {
    inner: Arc<Mutex<AContextInner<S>>>,
}

impl<S: Float> AContext<S> {
    /// Create a new AContext
    pub fn new() -> Self {
        AContext {
            inner: AContextInner::new(),
        }
    }

    /// Get a context by its id
    pub fn from_cid(cid: usize) -> Option<Self> {
        CONTEXT_MAP
            .lock()
            .unwrap()
            .get(&cid)
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast().ok())
            .map(|inner| AContext { inner })
    }

    /// Get the context id
    pub fn cid(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.cid()
    }

    /// Mark a variable as independent
    pub fn set_indep(&mut self, x: &mut AFloat<S>) {
        let mut inner = self.inner.lock().unwrap();
        let vid = inner.vals.len();
        inner.vals.push(NumCast::from(x.value()).unwrap());
        x.set_context(inner.cid(), vid);
        inner.indeps.push(vid);
    }

    /// Mark a variable as dependent
    pub fn set_dep(&mut self, x: &AFloat<S>) {
        let mut inner = self.inner.lock().unwrap();
        let vid = match x.context() {
            Some((cid, vid)) => {
                assert_eq!(cid, inner.cid());
                vid
            }
            None => {
                // Record constant
                let vid = inner.vals.len();
                inner.vals.push(NumCast::from(x.value()).unwrap());
                inner.ops.push(Operation {
                    opcode: OpCode::Const,
                    vid,
                    arg1: None,
                    arg2: None,
                });
                vid
            }
        };
        inner.deps.push(vid);
    }

    /// Create idependent var
    pub fn new_indep(&mut self, value: S) -> AFloat<S> {
        let mut indep = AFloat::new(value, S::zero());
        self.set_indep(&mut indep);
        indep
    }

    /// Create idependent vector
    pub fn new_indep_vec(&mut self, length: usize, value: S) -> Vec<AFloat<S>> {
        let mut vec = vec![AFloat::new(value, S::zero()); length];
        for x in vec.iter_mut() {
            self.set_indep(x);
        }
        vec
    }

    /// Set slice dependent
    pub fn set_dep_slice(&mut self, slice: &[AFloat<S>]) {
        for x in slice.iter() {
            self.set_dep(x);
        }
    }

    /// Record an operation
    pub fn record(
        &mut self,
        opcode: OpCode,
        val: S,
        arg1: Option<usize>,
        arg2: Option<usize>,
    ) -> usize {
        let mut inner = self.inner.lock().unwrap();
        let vid = inner.vals.len();
        inner.vals.push(NumCast::from(val).unwrap());
        inner.ops.push(Operation {
            opcode,
            vid,
            arg1,
            arg2,
        });
        vid
    }

    /// Get a tape
    pub fn tape(&self) -> impl Tape<S> + Clone {
        let inner = self.inner.lock().unwrap();
        AContextTape {
            indeps: inner.indeps.clone(),
            deps: inner.deps.clone(),
            ops: inner.ops.clone(),
            vals: inner.vals.clone(),
        }
    }
}

impl<S: Float> Default for AContext<S> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct AContextTape<S: Float> {
    indeps: Vec<usize>,
    deps: Vec<usize>,
    ops: Vec<Operation>,
    vals: Vec<S>,
}

impl<S: Float> Tape<S> for AContextTape<S> {
    fn indeps(&self) -> &[usize] {
        &self.indeps
    }

    fn deps(&self) -> &[usize] {
        &self.deps
    }

    fn values(&self) -> &[S] {
        &self.vals
    }

    fn values_mut(&mut self) -> &mut [S] {
        &mut self.vals
    }

    fn ops_iter<'a>(&'a self) -> Box<dyn DoubleEndedIterator<Item = Operation> + 'a> {
        Box::new(self.ops.iter().cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acontext_new_and_drop() {
        let mut success = true;
        let cid = {
            let ctx = AContext::<f64>::new();
            let cid = ctx.cid();
            let ctx_map = CONTEXT_MAP.lock().unwrap();
            if !ctx_map.contains_key(&cid) {
                success = false;
            }
            cid
        };
        {
            let ctx_map = CONTEXT_MAP.lock().unwrap();
            if ctx_map.contains_key(&cid) {
                success = false;
            }
        }
        assert!(success);
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn acontext_record() {
        {
            let mut ctx = AContext::new();
            let mut a = AFloat::new(1.0, 0.0);
            let mut b = AFloat::new(2.0, 0.0);
            ctx.set_indep(&mut a);
            ctx.set_indep(&mut b);
            let c = a + b;
            ctx.set_dep(&c);
            let tape = ctx.tape();

            let ops = tape.ops_iter().collect::<Vec<_>>();
            assert_eq!(ops.len(), 1);
            let op = ops[0];
            assert_eq!(op.opcode, OpCode::Add);
            assert_eq!(op.arg1, Some(0));
            assert_eq!(op.arg2, Some(1));
            assert_eq!(op.vid, 2);

            let vals = tape.values();
            assert!((vals[0] - 1.0).abs() < std::f64::EPSILON);
            assert!((vals[1] - 2.0).abs() < std::f64::EPSILON);
            assert!((vals[2] - 3.0).abs() < std::f64::EPSILON);
        }
        {
            let mut ctx = AContext::new();
            let mut a = AFloat::new(1.0, 0.0);
            let b = AFloat::new(2.0, 0.0);
            ctx.set_indep(&mut a);
            let c = a + b;
            ctx.set_dep(&c);
            let tape = ctx.tape();

            let ops = tape.ops_iter().collect::<Vec<_>>();
            assert_eq!(ops.len(), 2);
            assert_eq!(ops[0].opcode, OpCode::Const);
            assert_eq!(ops[0].arg1, None);
            assert_eq!(ops[0].arg2, None);
            assert_eq!(ops[0].vid, 1);
            assert_eq!(ops[1].opcode, OpCode::Add);
            assert_eq!(ops[1].arg1, Some(0));
            assert_eq!(ops[1].arg2, Some(1));
            assert_eq!(ops[1].vid, 2);

            let vals = tape.values();
            assert!((vals[0] - 1.0).abs() < std::f64::EPSILON);
            assert!((vals[1] - 2.0).abs() < std::f64::EPSILON);
            assert!((vals[2] - 3.0).abs() < std::f64::EPSILON);
        }
        {
            let mut ctx = AContext::new();
            let a = AFloat::new(1.0, 0.0);
            let mut b = AFloat::new(2.0, 0.0);
            ctx.set_indep(&mut b);
            let c = a + b;
            ctx.set_dep(&c);
            let tape = ctx.tape();

            let ops = tape.ops_iter().collect::<Vec<_>>();
            assert_eq!(ops.len(), 2);
            assert_eq!(ops[0].opcode, OpCode::Const);
            assert_eq!(ops[0].arg1, None);
            assert_eq!(ops[0].arg2, None);
            assert_eq!(ops[0].vid, 1);
            assert_eq!(ops[1].opcode, OpCode::Add);
            assert_eq!(ops[1].arg1, Some(1));
            assert_eq!(ops[1].arg2, Some(0));
            assert_eq!(ops[1].vid, 2);

            let vals = tape.values();
            assert!((vals[0] - 2.0).abs() < std::f64::EPSILON);
            assert!((vals[1] - 1.0).abs() < std::f64::EPSILON);
            assert!((vals[2] - 3.0).abs() < std::f64::EPSILON);
        }
    }
}
