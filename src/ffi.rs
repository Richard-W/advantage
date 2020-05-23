#![allow(non_camel_case_types)]
use super::*;
use num::Zero;

// `AContext` bindings

#[no_mangle]
pub extern "C" fn adv_context_new() -> *mut AContext {
    Box::leak(Box::new(AContext::new()))
}

#[no_mangle]
pub unsafe extern "C" fn adv_context_free(this: *mut AContext) {
    Box::from_raw(this);
}

#[no_mangle]
pub extern "C" fn adv_context_new_independent(this: &'static mut AContext) -> *mut ADouble {
    Box::leak(Box::new(this.new_indep(0.0)))
}

#[no_mangle]
pub extern "C" fn adv_context_set_dependent(this: &mut AContext, val: &ADouble) {
    this.set_dep(val);
}

// `ADouble` bindings

#[no_mangle]
pub unsafe extern "C" fn adv_double_default() -> *mut ADouble {
    Box::leak(Box::new(ADouble::zero()))
}

#[no_mangle]
pub unsafe extern "C" fn adv_double_from_value(val: f64) -> *mut ADouble {
    Box::leak(Box::new(ADouble::from(val)))
}

#[no_mangle]
pub unsafe extern "C" fn adv_double_free(this: *mut ADouble) {
    Box::from_raw(this);
}

#[no_mangle]
pub extern "C" fn adv_double_copy(this: &ADouble) -> *mut ADouble {
    Box::leak(Box::new(this.clone()))
}

// `ADouble` operation bindings

macro_rules! binary_operation {
    ($op_name:ident, $op:tt) => {
        paste::item! {
            #[no_mangle]
            pub extern "C" fn [<adv_op_ $op_name>](a: &'static ADouble, b: &'static ADouble, result: &'static mut *mut ADouble) {
                *result = Box::leak(Box::new(*a $op *b));
            }
        }
    }
}

binary_operation!(add, +);
binary_operation!(sub, -);
binary_operation!(mul, *);
binary_operation!(div, /);

macro_rules! unary_function {
    ($func_name:ident) => {
        paste::item! {
            #[no_mangle]
            pub extern "C" fn [<adv_ $func_name>](x: &'static ADouble, result: &'static mut *mut ADouble) {
                *result = Box::leak(Box::new(x.$func_name()));
            }
        }
    }
}

unary_function!(sin);
unary_function!(cos);
unary_function!(tan);
unary_function!(abs);
unary_function!(exp);
unary_function!(ln);

macro_rules! binary_function {
    ($func_name:ident) => {
        paste::item! {
            #[no_mangle]
            pub extern "C" fn [<adv_ $func_name>](a: &'static ADouble, b: &'static ADouble, result: &'static mut *mut ADouble) {
                *result = Box::leak(Box::new(a.$func_name(*b)));
            }
        }
    }
}

binary_function!(min);
binary_function!(max);
