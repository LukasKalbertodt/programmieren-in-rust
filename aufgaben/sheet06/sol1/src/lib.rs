use std::ops::{Add, Mul};

#[cfg(test)]
mod tests;


/// Clamps a value into a given range. This function returns the value closest
/// to `value` which lies in between `min` and `max`.
///
/// *Note*: it's not clear whether `PartialOrd` or `Ord` is the correct bound
/// here. With `PartialEq`, some results may look strange to some.
/// `clamp(NaN, 0.0, 5.0)` would return `NaN` for example. `NaN` as min or max
/// wouldn't do anything.
pub fn clamp<T>(value: T, min: T, max: T) -> T
    where T: PartialOrd
{
    // This is a small little trick. We want to avoid using if-else here, so
    // we match the unit value `()` (void) and use the match guards.
    match () {
        () if value < min => min,
        () if value > max => max,
        _ => value,
    }
}


/// Returns the sum and the product of the two given parameters.
///
/// *Note*: Either a Clone or Copy bound is necessary. Clone was choosen here,
/// because it's more general.
pub fn sum_product<T, U>(a: T, b: U)
    -> (<T as Add<U>>::Output, <T as Mul<U>>::Output)
    where T: Add<U> + Mul<U> + Clone,
          U: Clone
{
    (a.clone() + b.clone(), a * b)
}

/// Extension trait for simple conversion from `bool` to `Option<T>`
pub trait BoolOptionExt {
    /// If `self` is `true`, `Some(value)` is returned, `None` otherwise.
    fn into_option<T>(self, value: T) -> Option<T>;
}

impl BoolOptionExt for bool {
    fn into_option<T>(self, value: T) -> Option<T> {
        match self {
            true => Some(value),
            false =>  None,
        }
    }
}
