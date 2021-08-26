// Copyright 2021 Thomas Ballasi
// Copyright 2021 Stéphane Traut
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::easing::Easing;
use crate::utils::Number;
use crate::Event;

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line_static() {
        let rotate_event_neg: Rotate = (0, -1).into();
        assert_eq!(rotate_event_neg.to_line(), " R,0,0,,-1");

        let mut rotate_event_depth: Rotate = (0, 1).into();
        rotate_event_depth.set_depth(2);
        assert_eq!(rotate_event_depth.to_line(), "   R,0,0,,1");
    }

    #[test]
    fn to_line_dynamic() {
        let rotate_event: Rotate = (0, 1000, 0, 1).into();
        assert_eq!(rotate_event.to_line(), " R,0,0,1000,0,1");

        let rotate_event_easing: Rotate = (Easing::QuadOut, 0, 1000, 0, 1).into();
        assert_eq!(rotate_event_easing.to_line(), " R,4,0,1000,0,1");
    }

    #[test]
    fn to_line_dynamic_float() {
        let rotate_event: Rotate = (0, 1000, 0.25, 0.75).into();
        assert_eq!(rotate_event.to_line(), " R,0,0,1000,0.25,0.75");

        let rotate_event_easing: Rotate = (Easing::QuadOut, 0, 1000, 0.25, 0.75).into();
        assert_eq!(rotate_event_easing.to_line(), " R,4,0,1000,0.25,0.75");
    }
}

/// `Rotate` event
pub enum Rotate {
    Static(usize, i32, Number),
    Dynamic(usize, Easing, i32, i32, Number, Number),
}

impl Event for Rotate {
    fn to_line(&self) -> String {
        match self {
            Rotate::Static(depth, time, value) => format!(
                "{} R,{},{},,{}",
                " ".repeat(*depth),
                Easing::Linear.id(),
                time,
                value
            ),
            Rotate::Dynamic(depth, easing, start_time, end_time, start_value, end_value) => {
                format!(
                    "{} R,{},{},{},{},{}",
                    " ".repeat(*depth),
                    easing.id(),
                    start_time,
                    end_time,
                    start_value,
                    end_value
                )
            }
        }
    }

    fn set_depth(&mut self, depth: usize) {
        match self {
            Rotate::Static(ref mut current_depth, ..) => *current_depth = depth,
            Rotate::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }
}

/// Creates a static `Rotate` event with the timestamp and the value of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{event::Rotate, Sprite};
///
/// let time = 0;
/// let value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.rotate_((time, value));
/// ```
impl<T> Into<Rotate> for (i32, T)
where
    T: Into<Number>,
{
    fn into(self) -> Rotate {
        Rotate::Static(0, self.0, self.1.into())
    }
}

/// Creates a dynamic `Rotate` event with the timestamps and the values of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{event::Rotate, Sprite};
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_value = 0;
/// let end_value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.rotate_((start_time, end_time, start_value, end_value));
/// ```
impl<T, U> Into<Rotate> for (i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Rotate {
        Rotate::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            self.2.into(),
            self.3.into(),
        )
    }
}

/// Creates a dynamic `Rotate` event with the easing, the timestamps and the values of the element
///
/// Example:
/// ```
/// use osb::{event::Rotate, Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_value = 0;
/// let end_value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.rotate_((easing, start_time, end_time, start_value, end_value));
/// ```
impl<T, U> Into<Rotate> for (Easing, i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Rotate {
        Rotate::Dynamic(0, self.0, self.1, self.2, self.3.into(), self.4.into())
    }
}
