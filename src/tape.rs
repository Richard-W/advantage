use super::*;
use std::fmt;
use std::iter::{DoubleEndedIterator, Iterator};

/// Evaluation procedure and intermediate values of a function evaluation
pub trait Tape<S: Float + 'static>: Send + Sync + fmt::Debug {
    /// Independent variable indices
    fn indeps(&self) -> &[usize];
    /// Dependent variable indices
    fn deps(&self) -> &[usize];
    /// Get intermediate values
    fn values(&self) -> &[S];
    /// Get intermediate values (mutable)
    fn values_mut(&mut self) -> &mut [S];
    /// Iterate through operations
    fn ops_iter<'a>(&'a self) -> Box<dyn DoubleEndedIterator<Item = Operation> + 'a>;
}

/// Extra functions on a tape
pub trait TapeExt<S: Float + 'static> {
    /// Number of independents
    fn num_indeps(&self) -> usize;

    /// Number of dependents
    fn num_deps(&self) -> usize;

    /// Number of abs-function operations
    fn num_abs(&self) -> usize;

    /// Maximum value idx on this tape
    fn max_id(&self) -> usize;

    /// Stored arguments to the function
    fn x(&self) -> DVector<S>
    where
        S: fmt::Debug;

    /// Stored result of the function
    fn y(&self) -> DVector<S>
    where
        S: fmt::Debug;

    /// Re-evaluate function from stored evaluation procedure
    fn zero_order(&mut self, x: &DVector<S>)
    where
        S: fmt::Debug;

    /// Calculate adjoint of Jacobian
    fn first_order_forward(&self, dx: &DVector<S>) -> DVector<S>
    where
        S: fmt::Debug;

    /// Calculate reverse-adjoint of Jacobian
    fn first_order_reverse(&self, ybar: &DVector<S>) -> DVector<S>
    where
        S: fmt::Debug;
}

impl<T, S: Float + 'static> TapeExt<S> for T
where
    T: Tape<S> + ?Sized,
{
    fn num_indeps(&self) -> usize {
        self.indeps().len()
    }

    fn num_deps(&self) -> usize {
        self.deps().len()
    }

    fn num_abs(&self) -> usize {
        self.ops_iter()
            .filter(|op| op.opcode == OpCode::Abs)
            .count()
    }

    fn max_id(&self) -> usize {
        let indep_max = self.indeps().iter().cloned().max().unwrap_or(0);
        let dep_max = self.deps().iter().cloned().max().unwrap_or(0);
        let op_max = self.ops_iter().map(|op| op.vid).max().unwrap_or(0);
        indep_max.max(dep_max).max(op_max)
    }

    fn x(&self) -> DVector<S>
    where
        S: fmt::Debug,
    {
        let mut x = DVector::zeros(self.num_indeps());
        for (idx, vid) in self.indeps().iter().enumerate() {
            x[idx] = self.values()[*vid];
        }
        x
    }

    fn y(&self) -> DVector<S>
    where
        S: fmt::Debug,
    {
        let mut y = DVector::zeros(self.num_deps());
        for (idx, vid) in self.deps().iter().enumerate() {
            y[idx] = self.values()[*vid];
        }
        y
    }

    fn zero_order(&mut self, x: &DVector<S>)
    where
        S: fmt::Debug,
    {
        assert_eq!(x.nrows(), self.num_indeps());
        let indeps = self.indeps().to_vec();
        let ops = self.ops_iter().collect::<Vec<_>>();
        let values = self.values_mut();
        for (idx, vid) in indeps.into_iter().enumerate() {
            values[vid] = x[idx];
        }
        for op in ops.into_iter() {
            op.zero_order(values);
        }
    }

    fn first_order_forward(&self, dx: &DVector<S>) -> DVector<S>
    where
        S: fmt::Debug,
    {
        let v = self.values();
        let mut dv = vec![S::zero(); v.len()];
        for (idx, vid) in self.indeps().iter().enumerate() {
            dv[*vid] = dx[idx];
        }
        for op in self.ops_iter() {
            op.first_order(v, &mut dv);
        }
        let mut dy = DVector::zeros(self.num_deps());
        for (idx, vid) in self.deps().iter().enumerate() {
            dy[idx] = dv[*vid];
        }
        dy
    }

    fn first_order_reverse(&self, ybar: &DVector<S>) -> DVector<S>
    where
        S: fmt::Debug,
    {
        let v = self.values();
        let mut vbar = vec![S::zero(); v.len()];
        for (idx, vid) in self.deps().iter().enumerate() {
            vbar[*vid] = ybar[idx];
        }
        for op in self.ops_iter().rev() {
            op.first_order_reverse(v, &mut vbar);
        }
        let mut xbar = DVector::zeros(self.num_indeps());
        for (idx, vid) in self.indeps().iter().enumerate() {
            xbar[idx] = vbar[*vid];
        }
        xbar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    adv_fn! {
        /// Function using all arithmetic operations
        fn all_arithmetic_test_func(x: [[1]]) -> [[1]] {
            let x = x[0];
            let v1 = x + x - x * x / x;
            let v2 = (v1 + 2.0 - 2.0) * 2.0 / 2.0;
            let v3 = -(2.0 - (2.0 + v2));
            let v4 = 2.0 / (2.0 * v3);
            let v5 = 1.0 / v4;
            adv_dvec!(v5)
        }
    }

    /// `all_arithmetic_test_func` can be replayed from tape with different inputs
    #[test]
    fn zero_order_arithmetic() {
        let mut tape = adv_fn_obj!(all_arithmetic_test_func).tape(&DVector::zeros(1));
        tape.zero_order(&DVector::from_vec(vec![3.0]));
        let actual = tape.y()[0];
        let expected = all_arithmetic_test_func(DVector::from_vec(vec![3.0]))[0];
        assert!((actual - expected).abs() < std::f64::EPSILON);
    }

    /// Forward-mode AD works on `all_arithmetic_test_func`
    #[test]
    fn first_order_forward_arithmetic() {
        let tape = adv_fn_obj!(all_arithmetic_test_func).tape(&DVector::from_element(1, 3.0));
        let dy = tape.first_order_forward(&DVector::from_element(1, 1.0));
        assert!((dy[0] - 1.0).abs() < std::f64::EPSILON);
    }

    /// Reverse-mode AD works on `all_arithmetic_test_func`
    #[test]
    fn first_order_reverse_arithmetic() {
        let tape = adv_fn_obj!(all_arithmetic_test_func).tape(&DVector::from_element(1, 3.0));
        let xbar = tape.first_order_reverse(&DVector::from_element(1, 1.0));
        assert!((xbar[0] - 1.0).abs() < std::f64::EPSILON);
    }

    /// Test forward-mode and reverse-mode on a single unary function
    macro_rules! unary_test_case {
        ($func:ident, $start:expr, $end:expr, $num_tests:expr, $dy:expr) => {{
            const EPS: f64 = 1e-5;

            let num_tests: usize = $num_tests;
            let start: f64 = $start;
            let end: f64 = $end;

            let dx = (end - start) / (num_tests as f64);
            for i in 0..num_tests {
                let x = start + (i as f64) * dx;
                println!("Test case {}({})", stringify!($func), x);
                let tape = {
                    let mut ctx = AContext::new();
                    let mut x = AFloat::<f64>::new(x, 0.0);
                    ctx.set_indep(&mut x);
                    let y = x.$func();
                    ctx.set_dep(&y);
                    ctx.tape()
                };

                let dx = DVector::from_element(1, 1.0);
                let dy = tape.first_order_forward(&dx);
                assert!((dy[0] - ($dy)(x)).abs() < EPS);

                let ybar = DVector::from_element(1, 1.0);
                let xbar = tape.first_order_reverse(&ybar);
                assert!((xbar[0] - ($dy)(x)).abs() < EPS);
            }
        }};
        ($func:ident, $dy:expr) => {
            unary_test_case!($func, 0.0, 1.0, 10, $dy);
        };
    }

    /// Forward-mode and reverse-mode work on nonlinear unary functions
    #[test]
    #[allow(clippy::redundant_closure_call)]
    #[allow(clippy::cognitive_complexity)]
    fn first_order_unary_nonlinear_functions() {
        unary_test_case!(sin, |x: f64| x.cos());
        unary_test_case!(cos, |x: f64| -x.sin());
        unary_test_case!(tan, |x: f64| (1.0 / x.cos()).powi(2));
        unary_test_case!(exp, |x: f64| x.exp());
        unary_test_case!(ln, 0.1, 1.0, 9, |x: f64| 1.0 / x);
        unary_test_case!(sqrt, 0.1, 1.0, 9, |x: f64| 0.5 / x.sqrt());
        unary_test_case!(asin, |x: f64| 1.0 / (1.0 - x.powi(2)).sqrt());
        unary_test_case!(acos, |x: f64| -1.0 / (1.0 - x.powi(2)).sqrt());
        unary_test_case!(atan, |x: f64| 1.0 / (1.0 + x.powi(2)));
    }
}
