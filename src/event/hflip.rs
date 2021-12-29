use crate::easing::Easing;
use crate::Event;

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line() {
        let hflip_event: HFlip = (0, 1000).into();
        assert_eq!(hflip_event.to_line(), " P,0,0,1000,H");

        let mut hflip_event_depth: HFlip = (Easing::QuadOut, 0, 1000).into();
        hflip_event_depth.set_depth(2);
        assert_eq!(hflip_event_depth.to_line(), "   P,4,0,1000,H");
    }
}

/// `HFlip` event
pub enum HFlip {
    Dynamic(usize, Easing, i32, i32),
}

impl Event for HFlip {
    fn to_line(&self) -> String {
        match self {
            HFlip::Dynamic(depth, easing, start_time, end_time) => {
                format!(
                    "{} P,{},{},{},H",
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
            HFlip::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            HFlip::Dynamic(_, _, start_time, _) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            HFlip::Dynamic(_, _, _, end_time) => *end_time,
        }
    }
}

/// Creates a `HFlip` event with the timestamps
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
/// sprite.hflip_((start_time, end_time));
/// ```
impl Into<HFlip> for (i32, i32) {
    fn into(self) -> HFlip {
        HFlip::Dynamic(0, Easing::Linear, self.0, self.1)
    }
}

/// Creates a `HFlip` event with the easing and the timestamps
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
/// sprite.hflip_((easing, start_time, end_time));
/// ```
impl Into<HFlip> for (Easing, i32, i32) {
    fn into(self) -> HFlip {
        HFlip::Dynamic(0, self.0, self.1, self.2)
    }
}
