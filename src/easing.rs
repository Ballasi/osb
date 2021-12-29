use crate::utils::Number;
use std::error::Error;
use std::fmt;
use std::f32::consts::PI;

/// `Easing`s as defined in the [official osu! specifications](https://osu.ppy.sh/wiki/en/Storyboard_Scripting/Commands)
///
/// If you're interested in learning more about easing functions, how they work and what they are corresponding to, we'd suggest you take a look at [easing.net](https://easings.net/)
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Easing {
    /// The default `Easing` on osu!'s official editor
    Linear,
    /// The changes happen fast at first, but then slow down toward the end
    Out,
    /// The changes happen slowly at first, but then speed up toward the end
    In,
    /// Same as `Easing::In`
    QuadIn,
    /// Same as `Easing::Out`
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    SineIn,
    SineOut,
    SineInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    ElasticIn,
    ElasticOut,
    /// Same as `Easing::ElasticOut`
    ElasticHalfOut,
    /// Same as `Easing::ElasticOut`
    ElasticQuarterOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

/// The error type returned when parsing an `Easing` failed
///
/// Example:
/// ```
/// use osb::{Easing, EasingParsingError};
/// assert_eq!(Easing::get_easing(42), Err(EasingParsingError::IncorrectID));
/// ```
#[derive(Debug, PartialEq)]
pub enum EasingParsingError {
    IncorrectID,
}

impl fmt::Display for EasingParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Given easing ID does not correspond to any existing easing")
    }
}

impl Error for EasingParsingError {}

impl PartialEq for Easing {
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.
    ///
    /// Some easing, in osu!'s implementation, are visually similar despite having a different `id` or name.
    /// Therefore, these easing are considered equal by the `==` binary operator too.
    ///
    /// Example:
    /// ```
    /// use osb::Easing;
    /// assert_eq!(Easing::Out, Easing::QuadOut);
    /// assert_ne!(Easing::Out, Easing::In);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Easing::Out, Easing::QuadOut) => true,
            (Easing::QuadOut, Easing::Out) => true,
            (Easing::In, Easing::QuadIn) => true,
            (Easing::QuadIn, Easing::In) => true,
            (Easing::ElasticOut, Easing::ElasticHalfOut) => true,
            (Easing::ElasticHalfOut, Easing::ElasticOut) => true,
            (Easing::ElasticOut, Easing::ElasticQuarterOut) => true,
            (Easing::ElasticQuarterOut, Easing::ElasticOut) => true,
            (Easing::ElasticHalfOut, Easing::ElasticQuarterOut) => true,
            (Easing::ElasticQuarterOut, Easing::ElasticHalfOut) => true,
            (x, y) => *x as u8 == *y as u8,
        }
    }
}

impl Easing {
    /// A method to retrieve an `Easing` type from an `id` as defined in the osu!'s specifications
    ///
    /// Example:
    /// ```
    /// use osb::{Easing, EasingParsingError};
    /// assert_eq!(Easing::get_easing(0), Ok(Easing::Linear));
    /// assert_eq!(Easing::get_easing(42), Err(EasingParsingError::IncorrectID));
    /// ```
    pub fn get_easing(id: u8) -> Result<Easing, EasingParsingError> {
        match id {
            0 => Ok(Easing::Linear),
            1 => Ok(Easing::QuadOut),
            2 => Ok(Easing::QuadIn),
            3 => Ok(Easing::QuadIn),
            4 => Ok(Easing::QuadOut),
            5 => Ok(Easing::QuadInOut),
            6 => Ok(Easing::CubicIn),
            7 => Ok(Easing::CubicOut),
            8 => Ok(Easing::CubicInOut),
            9 => Ok(Easing::QuartIn),
            10 => Ok(Easing::QuartOut),
            11 => Ok(Easing::QuartInOut),
            12 => Ok(Easing::QuintIn),
            13 => Ok(Easing::QuintOut),
            14 => Ok(Easing::QuintInOut),
            15 => Ok(Easing::SineIn),
            16 => Ok(Easing::SineOut),
            17 => Ok(Easing::SineInOut),
            18 => Ok(Easing::ExpoIn),
            19 => Ok(Easing::ExpoOut),
            20 => Ok(Easing::ExpoInOut),
            21 => Ok(Easing::CircIn),
            22 => Ok(Easing::CircOut),
            23 => Ok(Easing::CircInOut),
            24 => Ok(Easing::ElasticIn),
            25 => Ok(Easing::ElasticOut),
            26 => Ok(Easing::ElasticOut),
            27 => Ok(Easing::ElasticOut),
            28 => Ok(Easing::ElasticInOut),
            29 => Ok(Easing::BackIn),
            30 => Ok(Easing::BackOut),
            31 => Ok(Easing::BackInOut),
            32 => Ok(Easing::BounceIn),
            33 => Ok(Easing::BounceOut),
            34 => Ok(Easing::BounceInOut),
            _ => Err(EasingParsingError::IncorrectID),
        }
    }

    /// Returns the `id` of an `Easing`
    ///
    /// Example:
    /// ```
    /// use osb::Easing;
    /// assert_eq!(Easing::Linear.id(), 0);
    /// ```
    pub fn id(self) -> u8 {
        self as u8
    }

    /// Returns the value of an `Easing` at a certain time
    ///
    /// Example:
    /// ```
    /// use osb::Easing;
    ///
    /// // Let's say we have a MoveX event happening between the timestamps 0ms and 2000ms. This
    /// // event uses a Out easing and the sprite moves from the X position 100 to 200. What is
    /// // the X position of the sprite at the timestamp 1000ms?
    /// let value = Easing::Out.ease(1000, 0, 2000, 100., 200.);
    /// assert_eq!(value, Some(175.));
    /// ```
    pub fn ease(
        self,
        time: i32,
        start_time: i32,
        end_time: i32,
        from: impl Into<Number>,
        to: impl Into<Number>,
    ) -> Option<f32>
    {
        let from = from.into().as_f32();
        let to = to.into().as_f32();

        if time < start_time || time > end_time || to < from {
            return None;
        }

        Some(
            self.calculate((time - start_time) as f32 / (end_time - start_time) as f32)
                * (to - from)
                + from,
        )
    }

    fn calculate(self, x: f32) -> f32 {
        if x < f32::EPSILON {
            // if x < 0.
            return 0.;
        }

        if 1. - x < f32::EPSILON {
            // if x > 1.
            return 1.;
        }

        match self {
            Easing::Linear => x,
            Easing::In | Easing::QuadIn => x * x,
            Easing::Out | Easing::QuadOut => Easing::In.reverse(x),
            Easing::QuadInOut => Easing::In.in_out(x),
            Easing::CubicIn => x * x * x,
            Easing::CubicOut => Easing::CubicIn.reverse(x),
            Easing::CubicInOut => Easing::CubicIn.in_out(x),
            Easing::QuartIn => x * x * x * x,
            Easing::QuartOut => Easing::QuartIn.reverse(x),
            Easing::QuartInOut => Easing::QuartIn.in_out(x),
            Easing::QuintIn => x * x * x * x * x,
            Easing::QuintOut => Easing::QuintIn.reverse(x),
            Easing::QuintInOut => Easing::QuintIn.in_out(x),
            Easing::SineIn => 1. - (x * PI / 2.).cos(),
            Easing::SineOut => Easing::SineIn.reverse(x),
            Easing::SineInOut => Easing::SineIn.in_out(x),
            Easing::ExpoIn => 2.0_f32.powf(10. * (x - 1.)),
            Easing::ExpoOut => Easing::ExpoIn.reverse(x),
            Easing::ExpoInOut => Easing::ExpoIn.in_out(x),
            Easing::CircIn => 1. - (1. - x * x).sqrt(),
            Easing::CircOut => Easing::CircIn.reverse(x),
            Easing::CircInOut => Easing::CircOut.in_out(x),
            Easing::ElasticIn => Easing::ElasticOut.reverse(x),
            Easing::ElasticOut | Easing::ElasticHalfOut | Easing::ElasticQuarterOut => {
                2.0_f32.powf(-10. * x) * ((x - 0.075) * 2. * PI / 0.3).sin() + 1.
            }
            Easing::ElasticInOut => Easing::ElasticIn.in_out(x),
            Easing::BackIn => x * x * ((1.70158 + 1.) * x - 1.70158),
            Easing::BackOut => Easing::BackIn.reverse(x),
            Easing::BackInOut => Easing::BackIn.in_out(x),
            Easing::BounceIn => Easing::BounceOut.reverse(x),
            Easing::BounceOut => {
                if x < 1. / 2.75 {
                    7.5625 * x * x
                } else if x < 2. / 2.75 {
                    7.5625 * (x - 1.5 / 2.75) * (x - 1.5 / 2.75) + 0.75
                } else if x < 2.5 / 2.75 {
                    7.5625 * (x - 2.5 / 2.75) * (x - 2.5 / 2.75) + 0.9375
                } else {
                    7.5625 * (x - 2.625 / 2.75) * (x - 2.625 / 2.75) + 0.984375
                }
            }
            Easing::BounceInOut => Easing::BounceIn.in_out(x),
        }
    }

    fn reverse(self, x: f32) -> f32 {
        1. - self.calculate(1. - x)
    }

    fn in_out(self, x: f32) -> f32 {
        0.5 * if x < 0.5 {
            self.calculate(2. * x)
        } else {
            2. - self.calculate(2. - 2. * x)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Easing;

    #[test]
    fn get_easing() {
        assert_eq!(Easing::get_easing(1), Ok(Easing::QuadOut));
        assert_eq!(Easing::get_easing(2), Ok(Easing::QuadIn));
        assert_eq!(Easing::get_easing(3), Ok(Easing::QuadIn));
        assert_eq!(Easing::get_easing(4), Ok(Easing::QuadOut));
        assert_eq!(Easing::get_easing(5), Ok(Easing::QuadInOut));
        assert_eq!(Easing::get_easing(6), Ok(Easing::CubicIn));
        assert_eq!(Easing::get_easing(7), Ok(Easing::CubicOut));
        assert_eq!(Easing::get_easing(8), Ok(Easing::CubicInOut));
        assert_eq!(Easing::get_easing(9), Ok(Easing::QuartIn));
        assert_eq!(Easing::get_easing(10), Ok(Easing::QuartOut));
        assert_eq!(Easing::get_easing(11), Ok(Easing::QuartInOut));
        assert_eq!(Easing::get_easing(12), Ok(Easing::QuintIn));
        assert_eq!(Easing::get_easing(13), Ok(Easing::QuintOut));
        assert_eq!(Easing::get_easing(14), Ok(Easing::QuintInOut));
        assert_eq!(Easing::get_easing(15), Ok(Easing::SineIn));
        assert_eq!(Easing::get_easing(16), Ok(Easing::SineOut));
        assert_eq!(Easing::get_easing(17), Ok(Easing::SineInOut));
        assert_eq!(Easing::get_easing(18), Ok(Easing::ExpoIn));
        assert_eq!(Easing::get_easing(19), Ok(Easing::ExpoOut));
        assert_eq!(Easing::get_easing(20), Ok(Easing::ExpoInOut));
        assert_eq!(Easing::get_easing(21), Ok(Easing::CircIn));
        assert_eq!(Easing::get_easing(22), Ok(Easing::CircOut));
        assert_eq!(Easing::get_easing(23), Ok(Easing::CircInOut));
        assert_eq!(Easing::get_easing(24), Ok(Easing::ElasticIn));
        assert_eq!(Easing::get_easing(25), Ok(Easing::ElasticOut));
        assert_eq!(Easing::get_easing(26), Ok(Easing::ElasticOut));
        assert_eq!(Easing::get_easing(27), Ok(Easing::ElasticOut));
        assert_eq!(Easing::get_easing(28), Ok(Easing::ElasticInOut));
        assert_eq!(Easing::get_easing(29), Ok(Easing::BackIn));
        assert_eq!(Easing::get_easing(30), Ok(Easing::BackOut));
        assert_eq!(Easing::get_easing(31), Ok(Easing::BackInOut));
        assert_eq!(Easing::get_easing(32), Ok(Easing::BounceIn));
        assert_eq!(Easing::get_easing(33), Ok(Easing::BounceOut));
        assert_eq!(Easing::get_easing(34), Ok(Easing::BounceInOut));
    }

    #[test]
    fn easing_eq() {
        assert_eq!(Easing::Out, Easing::QuadOut);
        assert_eq!(Easing::QuadOut, Easing::Out);
        assert_eq!(Easing::In, Easing::QuadIn);
        assert_eq!(Easing::QuadIn, Easing::In);
        assert_eq!(Easing::ElasticOut, Easing::ElasticHalfOut);
        assert_eq!(Easing::ElasticHalfOut, Easing::ElasticOut);
        assert_eq!(Easing::ElasticOut, Easing::ElasticQuarterOut);
        assert_eq!(Easing::ElasticQuarterOut, Easing::ElasticOut);
        assert_eq!(Easing::ElasticHalfOut, Easing::ElasticQuarterOut);
        assert_eq!(Easing::ElasticQuarterOut, Easing::ElasticHalfOut);
    }

    #[test]
    fn ease_functions() {
        assert_eq!(Easing::CubicOut.ease(1, 0, 2, 0., 200.), Some(175.));
        assert_eq!(Easing::QuartOut.ease(1, 0, 2, 0., 200.), Some(187.5));
        assert_eq!(Easing::QuintOut.ease(1, 0, 2, 0., 200.), Some(193.75));
        assert_eq!(
            Easing::SineOut.ease(1, 0, 2, 0., 1.),
            Some(2_f32.sqrt() / 2.)
        );
        assert_eq!(Easing::ExpoOut.ease(1, 0, 2, 0., 200.), Some(193.75));
        assert_eq!(Easing::CircOut.ease(1, 0, 2, 0., 1.), Some(0.75_f32.sqrt()));
        assert_eq!(Easing::ElasticIn.ease(1, 0, 2, 0., 200.), Some(-3.125));
        assert_eq!(Easing::BackOut.ease(1, 0, 2, 0., 200.), Some(217.5395));
        assert_eq!(Easing::BounceIn.ease(1, 0, 2, 0., 200.), Some(46.875));
    }

    #[test]
    fn ease_functions_inout() {
        assert_eq!(Easing::QuadInOut.ease(1, 0, 4, 0., 40.), Some(5.));
        assert_eq!(Easing::QuadInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::QuartInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::CubicInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::QuintInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::SineInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::ExpoInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::CircInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::ElasticInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::BackInOut.ease(1, 0, 2, 0., 2.), Some(1.));
        assert_eq!(Easing::BounceInOut.ease(1, 0, 2, 0., 2.), Some(1.));
    }

    #[test]
    fn ease_out_of_bounds() {
        assert_eq!(Easing::Linear.ease(5, 0, 4, 0., 10.), None);
        assert_eq!(Easing::Linear.ease(2, 0, 4, 10., 5.), None);
    }
}
