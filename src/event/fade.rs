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
        let fade_event: Fade = (0, 1).into();
        assert_eq!(fade_event.to_line(), " F,0,0,,1");

        let mut fade_event_depth: Fade = (0, 1).into();
        fade_event_depth.set_depth(2);
        assert_eq!(fade_event_depth.to_line(), "   F,0,0,,1");
    }

    #[test]
    fn to_line_dynamic() {
        let fade_event: Fade = (0, 1000, 0, 1).into();
        assert_eq!(fade_event.to_line(), " F,0,0,1000,0,1");

        let fade_event_easing: Fade = (Easing::QuadOut, 0, 1000, 0, 1).into();
        assert_eq!(fade_event_easing.to_line(), " F,4,0,1000,0,1");
    }

    #[test]
    fn to_line_dynamic_float() {
        let fade_event: Fade = (0, 1000, 0.25, 0.75).into();
        assert_eq!(fade_event.to_line(), " F,0,0,1000,0.25,0.75");

        let fade_event_easing: Fade = (Easing::QuadOut, 0, 1000, 0.25, 0.75).into();
        assert_eq!(fade_event_easing.to_line(), " F,4,0,1000,0.25,0.75");
    }
}

/// `Fade` event
pub enum Fade {
    Static(usize, i32, Number),
    Dynamic(usize, Easing, i32, i32, Number, Number),
}

impl Event for Fade {
    fn to_line(&self) -> String {
        match self {
            Fade::Static(depth, time, value) => {
                format!(
                    "{} F,{},{},,{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    value
                )
            }
            Fade::Dynamic(depth, easing, start_time, end_time, start_value, end_value) => {
                format!(
                    "{} F,{},{},{},{},{}",
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
            Fade::Static(ref mut current_depth, ..) => *current_depth = depth,
            Fade::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            Fade::Static(_, start_time, _) => *start_time,
            Fade::Dynamic(_, _, start_time, ..) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            Fade::Static(_, end_time, _) => *end_time,
            Fade::Dynamic(_, _, _, end_time, ..) => *end_time,
        }
    }
}

/// Creates a static `Fade` event with the timestamp and the opacity of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let time = 0;
/// let opacity = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.fade_((time, opacity));
/// ```
impl<T> Into<Fade> for (i32, T)
where
    T: Into<Number>,
{
    fn into(self) -> Fade {
        Fade::Static(0, self.0, self.1.into())
    }
}

/// Creates a dynamic `Fade` event with the timestamps and the opacity of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_opacity = 0;
/// let end_opacity = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.fade_((start_time, end_time, start_opacity, end_opacity));
/// ```
impl<T, U> Into<Fade> for (i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Fade {
        Fade::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            self.2.into(),
            self.3.into(),
        )
    }
}

/// Creates a dynamic `Fade` event with the easing, the timestamps and the opacity of the element
///
/// Example:
/// ```
/// use osb::{Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_opacity = 0;
/// let end_opacity = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.fade_((easing, start_time, end_time, start_opacity, end_opacity));
/// ```
impl<T, U> Into<Fade> for (Easing, i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Fade {
        Fade::Dynamic(0, self.0, self.1, self.2, self.3.into(), self.4.into())
    }
}
