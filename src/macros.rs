/// Replace a token by another token
///
/// Useful for counting repetitions in macros
#[macro_export]
#[doc(hidden)]
macro_rules! adv_replace {
    ($tt1:tt $tt2:expr) => {
        $tt2
    };
}

/// Define a function and automatically collect metadata
///
/// ## Example
/// ```
/// # extern crate advantage as adv;
/// # use adv::prelude::*;
/// adv_fn! {
///     fn ax1(v: [[3]], a: f64) -> [[3]] {
///         v.map(|x| a * x)
///     }
/// }
///
/// adv_fn! {
///     fn ax2([[x1, x2, x3]], a: f64) -> [[3]] {
///         adv_dvec![a * x1, a * x2, a * x3]
///     }
/// }
///
/// # fn main() {
/// let x = adv_dvec!(1.0, 2.0, 3.0);
/// let y = ax1(x, 2.0);
/// assert_eq!(y[0], 2.0);
/// assert_eq!(y[1], 4.0);
/// assert_eq!(y[2], 6.0);
///
/// let x = adv_dvec!(1.0, 2.0, 3.0);
/// let y = ax2(x, 2.0);
/// assert_eq!(y[0], 2.0);
/// assert_eq!(y[1], 4.0);
/// assert_eq!(y[2], 6.0);
/// # }
/// ```
#[macro_export]
macro_rules! adv_fn {
    {
        $(#[$attr:meta])*
        $vis:vis fn $func_name:ident ( $arg_name:ident : [[$($arg_dim:tt)*]] $( , $extra_arg:ident : $extra_type:ty )* $(,)? ) -> [[$($result_dim:tt)*]] {
            $($tt:tt)*
        }
    } => {
        $(#[$attr])*
        $vis fn $func_name<Scalar> ( $arg_name: $crate::DVector<Scalar> $(, $extra_arg : $extra_type )* ) -> $crate::DVector<Scalar>
        where
            Scalar: $crate::Float + From<f64> + $crate::Arithmetic<f64, Scalar> + $crate::ArithmeticAssign<f64>,
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
            $vis fn [< __adv_ $func_name >]<Scalar>($( $extra_arg : $extra_type ,)*) -> impl $crate::Function<Scalar>
            where
                Scalar: $crate::Float + From<f64> + $crate::Arithmetic<f64, Scalar> + $crate::ArithmeticAssign<f64>,
                f64: $crate::Arithmetic<Scalar, Scalar>,
            {
                $crate::SimpleFunction::new( $($arg_dim)*, $($result_dim)*, move |input| {
                    $func_name(input $(, $extra_arg.clone() )*)
                })
            }
        }
    };
    {
        $(#[$attr:meta])*
        $vis:vis fn $func_name:ident ( [[$($arg_name:ident),*]] $( , $extra_arg:ident : $extra_type:ty )* $(,)? ) -> [[$($result_dim:tt)*]] {
            $($tt:tt)*
        }
    } => {
        adv_fn! {
            $(#[$attr:meta])*
            $vis fn $func_name (__input: [[0 $(+ $crate::adv_replace!($arg_name 1usize))*]] $( , $extra_arg : $extra_type)* ) -> [[$($result_dim)*]] {
                let __arg_counter = 0;
                $(
                    let $arg_name = __input[__arg_counter];
                    let __arg_counter = __arg_counter + 1;
                )*
                $($tt)*
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
