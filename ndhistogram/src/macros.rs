/// Creates a [Histogram](crate::Histogram).
///
/// This macro is the recommended method for constructing new Histograms.
/// The arguments are a command separated list of [Axis](crate::axis::Axis).
/// Optionally, the type of the [Histogram](crate::Histogram) bin values may specified after a semi-colon after
/// the list of [Axis](crate::axis::Axis). If the bin value type is not specified, the default is f64.
///
/// # Example
///
/// ## Creating a 1D Histogram
/// ```rust
/// use ndhistogram::axis::Uniform;
/// use std::fmt::Display;
/// use ndhistogram::{ndhistogram, Histogram};
///
/// let hist1D = ndhistogram!(Uniform::new(10, -5.0, 5.0));
/// println!("{}", hist1D);
/// ```
///
/// ## Creating a 2D Histogram
/// ```rust
/// use ndhistogram::axis::{Uniform, Axis};
/// use ndhistogram::{ndhistogram, Histogram};
///
/// let hist2D = ndhistogram!(Uniform::new(10, -5.0, 5.0), Uniform::new(10, -5.0, 5.0));
/// // each axis has 10+2 bins due to under/overflow
/// assert_eq!(hist2D.axes().num_bins(), 12*12)
/// ```
///
/// ## Creating a Histogram with a specific bin value type
///
/// ```rust
/// use ndhistogram::axis::Uniform;
/// use ndhistogram::{ndhistogram, Histogram};
/// use ndhistogram::value::Mean;
///
/// let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0); Mean);
/// hist.fill_with(&0.0, 1.0);
/// hist.fill_with(&0.0, 3.0);
/// assert_eq!(hist.value(&0.0).unwrap().get(), 2.0) // mean of [1.0, 3.0] is 2.0
/// ```
#[macro_export]
macro_rules! ndhistogram {

    // ($( $x:expr ),+ $(,)*; $type:ident<$($generics:tt),+> $(;)*) => {
    //     {
    //         let axes = (
    //         $(
    //             $x,
    //         )*
    //     );
    //         $crate::VecHistogram::<_, $type<$($generics),+>>::new(axes)
    //     }
    // };

    ($( $x:expr ),+ $(,)*; $type:ty $(;)*) => {
        {
            let axes = (
            $(
                $x,
            )*
        );
            $crate::VecHistogram::<_, $type>::new(axes)
        }
    };
    ($( $x:expr ),+ $(,)*) => {
        {
            let axes = (
            $(
                $x,
            )*
        );
            $crate::VecHistogram::<_, f64>::new(axes)
        }
    };

}
