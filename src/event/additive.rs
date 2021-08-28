// Copyright 2021 Thomas Ballasi
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::easing::Easing;
use crate::Event;

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line() {
        let additive_event: Additive = (0, 1000).into();
        assert_eq!(additive_event.to_line(), " P,0,0,1000,A");

        let mut additive_event_depth: Additive = (Easing::QuadOut, 0, 1000).into();
        additive_event_depth.set_depth(2);
        assert_eq!(additive_event_depth.to_line(), "   P,4,0,1000,A");
    }
}

/// `Additive` event
pub enum Additive {
    Dynamic(usize, Easing, i32, i32),
}

impl Event for Additive {
    fn to_line(&self) -> String {
        match self {
            Additive::Dynamic(depth, easing, start_time, end_time) => {
                format!(
                    "{} P,{},{},{},A",
                    " ".repeat(*depth),
                    easing.id(),
                    start_time,
                    end_time,
                )
            }
        }
    }

    fn set_depth(&mut self, depth: usize) {
        match self {
            Additive::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            Additive::Dynamic(_, _, start_time, _) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            Additive::Dynamic(_, _, _, end_time) => *end_time,
        }
    }
}

/// Creates a `Additive` event with the timestamps
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let start_time = 0;
/// let end_time = 1000;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.additive_((start_time, end_time));
/// ```
impl Into<Additive> for (i32, i32) {
    fn into(self) -> Additive {
        Additive::Dynamic(0, Easing::Linear, self.0, self.1)
    }
}

/// Creates a `Additive` event with the easing and the timestamps
///
/// Example:
/// ```
/// use osb::{Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.additive_((easing, start_time, end_time));
/// ```
impl Into<Additive> for (Easing, i32, i32) {
    fn into(self) -> Additive {
        Additive::Dynamic(0, self.0, self.1, self.2)
    }
}
