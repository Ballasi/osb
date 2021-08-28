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
        let move_event_neg: Move = (-100, -320, -240).into();
        assert_eq!(move_event_neg.to_line(), " M,0,-100,,-320,-240");

        let mut move_event_depth: Move = (100, 0, 0).into();
        move_event_depth.set_depth(2);
        assert_eq!(move_event_depth.to_line(), "   M,0,100,,0,0");

        let move_event_vec2: Move = (0, 1000, Vec2::new(), Vec2::new()).into();
        assert_eq!(move_event_vec2.to_line(), " M,0,0,1000,0,0,0,0");
    }

    #[test]
    fn to_line_dynamic() {
        let move_event: Move = (0, 1000, 0, 0, 320, 240).into();
        assert_eq!(move_event.to_line(), " M,0,0,1000,0,0,320,240");

        let move_event_easing: Move = (Easing::QuadOut, 0, 1000, 0, 0, 320, 240).into();
        assert_eq!(move_event_easing.to_line(), " M,4,0,1000,0,0,320,240");
    }
}

/// `Move` event
pub enum Move {
    Static(usize, i32, Vec2),
    Dynamic(usize, Easing, i32, i32, Vec2, Vec2),
}

impl Event for Move {
    fn to_line(&self) -> String {
        match self {
            Move::Static(depth, time, pos) => {
                format!(
                    "{} M,{},{},,{},{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    pos.x,
                    pos.y
                )
            }
            Move::Dynamic(depth, easing, start_time, end_time, start_pos, end_pos) => {
                format!(
                    "{} M,{},{},{},{},{},{},{}",
                    " ".repeat(*depth),
                    easing.id(),
                    start_time,
                    end_time,
                    start_pos.x,
                    start_pos.y,
                    end_pos.x,
                    end_pos.y
                )
            }
        }
    }

    fn set_depth(&mut self, depth: usize) {
        match self {
            Move::Static(ref mut current_depth, ..) => *current_depth = depth,
            Move::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            Move::Static(_, start_time, _) => *start_time,
            Move::Dynamic(_, _, start_time, ..) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            Move::Static(_, end_time, _) => *end_time,
            Move::Dynamic(_, _, _, end_time, ..) => *end_time,
        }
    }
}

/// Creates a static `Move` event with the timestamp and the position of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
///
/// let time = 0;
/// let pos = Vec2::from(320, 240);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.move_((time, pos));
/// ```
impl Into<Move> for (i32, Vec2) {
    fn into(self) -> Move {
        Move::Static(0, self.0, self.1)
    }
}

/// Creates a static `Move` event with the timestamp and the position of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let time = 0;
/// let x = 320;
/// let y = 240;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.move_((time, x, y));
/// ```
impl<T, U> Into<Move> for (i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Move {
        Move::Static(0, self.0, Vec2::from(self.1, self.2))
    }
}

/// Creates a dynamic `Move` event with the timestamps and the positions of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_pos = Vec2::from(0, 0);
/// let end_pos = Vec2::from(320, 240);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.move_((start_time, end_time, start_pos, end_pos));
/// ```
impl Into<Move> for (i32, i32, Vec2, Vec2) {
    fn into(self) -> Move {
        Move::Dynamic(0, Easing::Linear, self.0, self.1, self.2, self.3)
    }
}

/// Creates a dynamic `Move` event with the timestamps and the positions of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_x = 0;
/// let start_y = 0;
/// let end_x = 320;
/// let end_y = 240;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.move_((start_time, end_time, start_x, start_y, end_x, end_y));
/// ```
impl<T, U, V, W> Into<Move> for (i32, i32, T, U, V, W)
where
    T: Into<Number>,
    U: Into<Number>,
    V: Into<Number>,
    W: Into<Number>,
{
    fn into(self) -> Move {
        Move::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            Vec2::from(self.2, self.3),
            Vec2::from(self.4, self.5),
        )
    }
}

/// Creates a dynamic `Move` event with the easing, the timestamps and the positions of the element
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_pos = Vec2::from(0, 0);
/// let end_pos = Vec2::from(320, 240);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.move_((easing, start_time, end_time, start_pos, end_pos));
/// ```
impl Into<Move> for (Easing, i32, i32, Vec2, Vec2) {
    fn into(self) -> Move {
        Move::Dynamic(0, self.0, self.1, self.2, self.3, self.4)
    }
}

/// Creates a dynamic `Move` event with the timestamps and the positions of the element
///
/// Example:
/// ```
/// use osb::{Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_x = 0;
/// let start_y = 0;
/// let end_x = 320;
/// let end_y = 240;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.move_((easing, start_time, end_time, start_x, start_y, end_x, end_y));
/// ```
impl<T, U, V, W> Into<Move> for (Easing, i32, i32, T, U, V, W)
where
    T: Into<Number>,
    U: Into<Number>,
    V: Into<Number>,
    W: Into<Number>,
{
    fn into(self) -> Move {
        Move::Dynamic(
            0,
            self.0,
            self.1,
            self.2,
            Vec2::from(self.3, self.4),
            Vec2::from(self.5, self.6),
        )
    }
}
