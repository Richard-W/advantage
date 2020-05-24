/// Define a function and automatically collect metadata
#[macro_export]
macro_rules! adv_fn {
    {
        $(#[$attr:meta])*
        $vis:vis fn $func_name:ident ( $arg_name:ident : [[$($arg_dim:tt)*]] $( , $extra_arg:ident : $extra_type:ty )* $(,)? ) -> [[$($result_dim:tt)*]] {
            $($tt:tt)*
        }
    } => {
        $(#[$attr])*
        $vis fn $func_name<Scalar: 'static> ( $arg_name: $crate::DVector<Scalar> $(, $extra_arg : $extra_type )* ) -> $crate::DVector<Scalar>
        where
            Scalar: core::fmt::Debug + $crate::Scalar<f64>,
            f64: $crate::Arithmetic<Scalar, Scalar>,
        {
            assert_eq!($arg_name.nrows(), $($arg_dim)*);
            let result = {
                $($tt)*
            };
            assert_eq!(result.nrows(), $($result_dim)*);
            result
        }

        // FIXME: If $func_name is imported from another module this function is not necessarily
        // visible
        $crate::paste::item! {
            #[doc(hidden)]
            $vis fn [< __adv_ $func_name >]($( $extra_arg : $extra_type ,)*) -> impl $crate::Function {
                $crate::SimpleFunction::new( $($arg_dim)*, $($result_dim)*, move |input| {
                    $func_name(input $(, $extra_arg.clone() )*)
                })
            }
        }
    };
}

/// Get the associated metadata for a function defined with `adv_fn!`
#[macro_export]
macro_rules! adv_fn_obj {
    ($name:ident $(, $extra_arg:expr )*) => {
        $crate::paste::expr! {
            [< __adv_ $name >]($($extra_arg ,)*)
        }
    }
}

/// Create a `DVector` containing the arguments
///
/// ## Example
/// ```
/// # extern crate advantage as adv;
/// # use adv::prelude::*;
///
/// # fn main() {
/// let vec = adv_dvec![1.0, 2.0, 3.0];
/// assert_eq!(vec.nrows(), 3);
/// assert_eq!(vec[0], 1.0);
/// assert_eq!(vec[1], 2.0);
/// assert_eq!(vec[2], 3.0);
///
/// let vec = adv_dvec![1.0; 3];
/// assert_eq!(vec.nrows(), 3);
/// assert_eq!(vec[0], 1.0);
/// assert_eq!(vec[1], 1.0);
/// assert_eq!(vec[2], 1.0);
/// # }
/// ```
#[macro_export]
macro_rules! adv_dvec {
    ($elem:expr; $n:expr) => ($crate::DVector::from_element($n, $elem));
    ($($x:expr),*) => ($crate::DVector::from_vec(vec![$($x),*]));
    ($($x:expr,)*) => ($crate::adv_dvec!($($x),*));
}
