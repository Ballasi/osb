// Copyright 2021 Thomas Ballasi
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::easing::Easing;
use crate::utils::{Number, Vec2};
use crate::Event;

#[cfg(test)]
mod tests {
    use crate::{event::*, utils::Vec2, Easing};

    #[test]
    fn to_line_static() {
        let scalevec_event_neg: ScaleVec = (-100, -1, -1).into();
        assert_eq!(scalevec_event_neg.to_line(), " V,0,-100,,-1,-1");

        let mut scalevec_event_depth: ScaleVec = (100, 1, 0.5).into();
        scalevec_event_depth.set_depth(2);
        assert_eq!(scalevec_event_depth.to_line(), "   V,0,100,,1,0.5");

        let scalevec_event_vec2: ScaleVec = (0, 1000, Vec2::new(), Vec2::from(1, 0)).into();
        assert_eq!(scalevec_event_vec2.to_line(), " V,0,0,1000,0,0,1,0");
    }

    #[test]
    fn to_line_dynamic() {
        let scalevec_event: ScaleVec = (0, 1000, 1, 1, 1, 0).into();
        assert_eq!(scalevec_event.to_line(), " V,0,0,1000,1,1,1,0");

        let scalevec_event_easing: ScaleVec = (Easing::QuadOut, 0, 1000, 1, 0, 1, 1).into();
        assert_eq!(scalevec_event_easing.to_line(), " V,4,0,1000,1,0,1,1");
    }
}

/// `ScaleVec` event
pub enum ScaleVec {
    Static(usize, i32, Vec2),
    Dynamic(usize, Easing, i32, i32, Vec2, Vec2),
}

impl Event for ScaleVec {
    fn to_line(&self) -> String {
        match self {
            ScaleVec::Static(depth, time, scale) => {
                format!(
                    "{} V,{},{},,{},{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    scale.x,
                    scale.y
                )
            }
            ScaleVec::Dynamic(depth, easing, start_time, end_time, start_scale, end_scale) => {
                format!(
                    "{} V,{},{},{},{},{},{},{}",
                    " ".repeat(*depth),
                    easing.id(),
                    start_time,
                    end_time,
                    start_scale.x,
                    start_scale.y,
                    end_scale.x,
                    end_scale.y
                )
            }
        }
    }

    fn set_depth(&mut self, depth: usize) {
        match self {
            ScaleVec::Static(ref mut current_depth, ..) => *current_depth = depth,
            ScaleVec::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            ScaleVec::Static(_, start_time, _) => *start_time,
            ScaleVec::Dynamic(_, _, start_time, ..) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            ScaleVec::Static(_, end_time, _) => *end_time,
            ScaleVec::Dynamic(_, _, _, end_time, ..) => *end_time,
        }
    }
}

/// Creates a static `ScaleVec` event with the timestamp and the scaling of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
///
/// let time = 0;
/// let scale = Vec2::from(1, 0.5);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scalevec_((time, scale));
/// ```
impl Into<ScaleVec> for (i32, Vec2) {
    fn into(self) -> ScaleVec {
        ScaleVec::Static(0, self.0, self.1)
    }
}

/// Creates a static `ScaleVec` event with the timestamp and the scaling of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let time = 0;
/// let x = 1;
/// let y = 0.5;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scalevec_((time, x, y));
/// ```
impl<T, U> Into<ScaleVec> for (i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> ScaleVec {
        ScaleVec::Static(0, self.0, Vec2::from(self.1, self.2))
    }
}

/// Creates a dynamic `ScaleVec` event with the timestamps and the scalings of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_scale = Vec2::from(1, 1);
/// let end_scale = Vec2::from(1, 0);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scalevec_((start_time, end_time, start_scale, end_scale));
/// ```
impl Into<ScaleVec> for (i32, i32, Vec2, Vec2) {
    fn into(self) -> ScaleVec {
        ScaleVec::Dynamic(0, Easing::Linear, self.0, self.1, self.2, self.3)
    }
}

/// Creates a dynamic `ScaleVec` event with the timestamps and the scalings of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_x = 1;
/// let start_y = 1;
/// let end_x = 1;
/// let end_y = 0;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scalevec_((start_time, end_time, start_x, start_y, end_x, end_y));
/// ```
impl<T, U, V, W> Into<ScaleVec> for (i32, i32, T, U, V, W)
where
    T: Into<Number>,
    U: Into<Number>,
    V: Into<Number>,
    W: Into<Number>,
{
    fn into(self) -> ScaleVec {
        ScaleVec::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            Vec2::from(self.2, self.3),
            Vec2::from(self.4, self.5),
        )
    }
}

/// Creates a dynamic `ScaleVec` event with the easing, the timestamps and the scalings of the element
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_scale = Vec2::from(1, 1);
/// let end_scale = Vec2::from(1, 0);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scalevec_((easing, start_time, end_time, start_scale, end_scale));
/// ```
impl Into<ScaleVec> for (Easing, i32, i32, Vec2, Vec2) {
    fn into(self) -> ScaleVec {
        ScaleVec::Dynamic(0, self.0, self.1, self.2, self.3, self.4)
    }
}

/// Creates a dynamic `ScaleVec` event with the timestamps and the scalings of the element
///
/// Example:
/// ```
/// use osb::{Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_x = 1;
/// let start_y = 1;
/// let end_x = 1;
/// let end_y = 0;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.scalevec_((easing, start_time, end_time, start_x, start_y, end_x, end_y));
/// ```
impl<T, U, V, W> Into<ScaleVec> for (Easing, i32, i32, T, U, V, W)
where
    T: Into<Number>,
    U: Into<Number>,
    V: Into<Number>,
    W: Into<Number>,
{
    fn into(self) -> ScaleVec {
        ScaleVec::Dynamic(
            0,
            self.0,
            self.1,
            self.2,
            Vec2::from(self.3, self.4),
            Vec2::from(self.5, self.6),
        )
    }
}
