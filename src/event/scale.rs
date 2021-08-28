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
        let scale_event_neg: Scale = (0, -1).into();
        assert_eq!(scale_event_neg.to_line(), " S,0,0,,-1");

        let mut scale_event_depth: Scale = (0, 1).into();
        scale_event_depth.set_depth(2);
        assert_eq!(scale_event_depth.to_line(), "   S,0,0,,1");
    }

    #[test]
    fn to_line_dynamic() {
        let scale_event: Scale = (0, 1000, 0, 1).into();
        assert_eq!(scale_event.to_line(), " S,0,0,1000,0,1");

        let scale_event_easing: Scale = (Easing::QuadOut, 0, 1000, 0, 1).into();
        assert_eq!(scale_event_easing.to_line(), " S,4,0,1000,0,1");
    }

    #[test]
    fn to_line_dynamic_float() {
        let scale_event: Scale = (0, 1000, 0.25, 0.75).into();
        assert_eq!(scale_event.to_line(), " S,0,0,1000,0.25,0.75");

        let scale_event_easing: Scale = (Easing::QuadOut, 0, 1000, 0.25, 0.75).into();
        assert_eq!(scale_event_easing.to_line(), " S,4,0,1000,0.25,0.75");
    }
}

/// `Scale` event
pub enum Scale {
    Static(usize, i32, Number),
    Dynamic(usize, Easing, i32, i32, Number, Number),
}

impl Event for Scale {
    fn to_line(&self) -> String {
        match self {
            Scale::Static(depth, time, value) => {
                format!(
                    "{} S,{},{},,{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    value
                )
            }
            Scale::Dynamic(depth, easing, start_time, end_time, start_value, end_value) => {
                format!(
                    "{} S,{},{},{},{},{}",
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
            Scale::Static(ref mut current_depth, ..) => *current_depth = depth,
            Scale::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            Scale::Static(_, start_time, _) => *start_time,
            Scale::Dynamic(_, _, start_time, ..) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            Scale::Static(_, end_time, _) => *end_time,
            Scale::Dynamic(_, _, _, end_time, ..) => *end_time,
        }
    }
}

/// Creates a static `Scale` event with the timestamp and the scale value of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{event::Scale, Sprite};
///
/// let time = 0;
/// let scale_value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scale_((time, scale_value));
/// ```
impl<T> Into<Scale> for (i32, T)
where
    T: Into<Number>,
{
    fn into(self) -> Scale {
        Scale::Static(0, self.0, self.1.into())
    }
}

/// Creates a dynamic `Scale` event with the timestamps and the scale values of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{event::Scale, Sprite};
///
/// let start_time = 0;
/// let end_time = 1000;
/// let scale_start_value = 0;
/// let scale_end_value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scale_((start_time, end_time, scale_start_value, scale_end_value));
/// ```
impl<T, U> Into<Scale> for (i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Scale {
        Scale::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            self.2.into(),
            self.3.into(),
        )
    }
}

/// Creates a dynamic `Scale` event with the easing, the timestamps and the scale values of the element
///
/// Example:
/// ```
/// use osb::{event::Scale, Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let scale_start_value = 0;
/// let scale_end_value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scale_((easing, start_time, end_time, scale_start_value, scale_end_value));
/// ```
impl<T, U> Into<Scale> for (Easing, i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Scale {
        Scale::Dynamic(0, self.0, self.1, self.2, self.3.into(), self.4.into())
    }
}
