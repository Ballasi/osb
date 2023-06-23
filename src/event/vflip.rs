use crate::easing::Easing;
use crate::Event;

/// `VFlip` event
#[derive(Clone)]
pub enum VFlip {
    Dynamic(usize, Easing, i32, i32),
}

impl Event for VFlip {
    fn to_line(&self) -> String {
        match self {
            VFlip::Dynamic(depth, easing, start_time, end_time) => {
                format!(
                    "{} P,{},{},{},V",
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
            VFlip::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            VFlip::Dynamic(_, _, start_time, _) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            VFlip::Dynamic(_, _, _, end_time) => *end_time,
        }
    }
}

/// Creates a `VFlip` event with the timestamps
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
/// sprite.vflip_((start_time, end_time));
/// ```
impl Into<VFlip> for (i32, i32) {
    fn into(self) -> VFlip {
        VFlip::Dynamic(0, Easing::Linear, self.0, self.1)
    }
}

/// Creates a `VFlip` event with the easing and the timestamps
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
/// sprite.vflip_((easing, start_time, end_time));
/// ```
impl Into<VFlip> for (Easing, i32, i32) {
    fn into(self) -> VFlip {
        VFlip::Dynamic(0, self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line() {
        let vflip_event: VFlip = (0, 1000).into();
        assert_eq!(vflip_event.to_line(), " P,0,0,1000,V");

        let mut vflip_event_depth: VFlip = (Easing::QuadOut, 0, 1000).into();
        vflip_event_depth.set_depth(2);
        assert_eq!(vflip_event_depth.to_line(), "   P,4,0,1000,V");
    }
}
