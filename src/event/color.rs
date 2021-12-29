// Copyright 2021 Thomas Ballasi
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::easing::Easing;
use crate::utils;
use crate::Event;

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line_static() {
        let mut color_event_depth: Color = (100, 0, 0, 0).into();
        color_event_depth.set_depth(2);
        assert_eq!(color_event_depth.to_line(), "   C,0,100,,0,0,0");
    }

    #[test]
    fn to_line_dynamic() {
        let color_event: Color = (0, 1000, 0, 0, 0, 255, 255, 255).into();
        assert_eq!(color_event.to_line(), " C,0,0,1000,0,0,0,255,255,255");

        let color_event_easing: Color = (Easing::QuadOut, 0, 1000, 0, 0, 0, 255, 255, 255).into();
        assert_eq!(
            color_event_easing.to_line(),
            " C,4,0,1000,0,0,0,255,255,255"
        );
    }
}

/// `Color` event
pub enum Color {
    Static(usize, i32, utils::Color),
    Dynamic(usize, Easing, i32, i32, utils::Color, utils::Color),
}

impl Event for Color {
    fn to_line(&self) -> String {
        match self {
            Color::Static(depth, time, color) => {
                format!(
                    "{} C,{},{},,{},{},{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    color.r(),
                    color.g(),
                    color.b(),
                )
            }
            Color::Dynamic(depth, easing, start_time, end_time, start_color, end_color) => {
                format!(
                    "{} C,{},{},{},{},{},{},{},{},{}",
                    " ".repeat(*depth),
                    easing.id(),
                    start_time,
                    end_time,
                    start_color.r(),
                    start_color.g(),
                    start_color.b(),
                    end_color.r(),
                    end_color.g(),
                    end_color.b(),
                )
            }
        }
    }

    fn set_depth(&mut self, depth: usize) {
        match self {
            Color::Static(ref mut current_depth, ..) => *current_depth = depth,
            Color::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            Color::Static(_, start_time, _) => *start_time,
            Color::Dynamic(_, _, start_time, ..) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            Color::Static(_, end_time, _) => *end_time,
            Color::Dynamic(_, _, _, end_time, ..) => *end_time,
        }
    }
}

/// Creates a static `Color` event with the timestamp and the colorization of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{utils::Color, Sprite};
///
/// let time = 0;
/// let color = Color::from(42, 42, 42);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.color_((time, color));
/// ```
impl Into<Color> for (i32, utils::Color) {
    fn into(self) -> Color {
        Color::Static(0, self.0, self.1)
    }
}

/// Creates a static `Color` event with the timestamp and the colorization of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let time = 0;
/// let r = 42;
/// let g = 42;
/// let b = 42;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.color_((time, r, g, b));
/// ```
impl Into<Color> for (i32, i32, i32, i32) {
    fn into(self) -> Color {
        Color::Static(0, self.0, utils::Color::from(self.1, self.2, self.3))
    }
}

/// Creates a dynamic `Color` event with the timestamps and the colorizations of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{utils::Color, Sprite};
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_color = Color::from(0, 0, 0);
/// let end_color = Color::from(255, 255, 255);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.color_((start_time, end_time, start_color, end_color));
/// ```
impl Into<Color> for (i32, i32, utils::Color, utils::Color) {
    fn into(self) -> Color {
        Color::Dynamic(0, Easing::Linear, self.0, self.1, self.2, self.3)
    }
}

/// Creates a dynamic `Color` event with the timestamps and the colorizations of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{Sprite};
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_r = 0;
/// let start_g = 0;
/// let start_b = 0;
/// let end_r = 255;
/// let end_g = 255;
/// let end_b = 255;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.color_((start_time, end_time, start_r, start_g, start_b, end_r, end_g, end_b));
/// ```
impl Into<Color> for (i32, i32, i32, i32, i32, i32, i32, i32) {
    fn into(self) -> Color {
        Color::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            utils::Color::from(self.2, self.3, self.4),
            utils::Color::from(self.5, self.6, self.7),
        )
    }
}

/// Creates a dynamic `Color` event with the easing, the timestamps and the colorizations of the element
///
/// Example:
/// ```
/// use osb::{utils::Color, Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_color = Color::from(0, 0, 0);
/// let end_color = Color::from(255, 255, 255);
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.color_((easing, start_time, end_time, start_color, end_color));
/// ```
impl Into<Color> for (Easing, i32, i32, utils::Color, utils::Color) {
    fn into(self) -> Color {
        Color::Dynamic(0, self.0, self.1, self.2, self.3, self.4)
    }
}

/// Creates a dynamic `Color` event with the timestamps and the colorizations of the element
///
/// Example:
/// ```
/// use osb::{Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_r = 0;
/// let start_g = 0;
/// let start_b = 0;
/// let end_r = 255;
/// let end_g = 255;
/// let end_b = 255;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.color_((easing, start_time, end_time, start_r, start_g, start_b, end_r, end_g, end_b));
/// ```
impl Into<Color> for (Easing, i32, i32, i32, i32, i32, i32, i32, i32) {
    fn into(self) -> Color {
        Color::Dynamic(
            0,
            self.0,
            self.1,
            self.2,
            utils::Color::from(self.3, self.4, self.5),
            utils::Color::from(self.6, self.7, self.8),
        )
    }
}
