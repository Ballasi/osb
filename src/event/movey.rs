use crate::easing::Easing;
use crate::utils::Number;
use crate::Event;

/// `MoveY` event
#[derive(Clone)]
pub enum MoveY {
    Static(usize, i32, Number),
    Dynamic(usize, Easing, i32, i32, Number, Number),
}

impl Event for MoveY {
    fn to_line(&self) -> String {
        match self {
            MoveY::Static(depth, time, value) => {
                format!(
                    "{} MY,{},{},,{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    value
                )
            }
            MoveY::Dynamic(depth, easing, start_time, end_time, start_value, end_value) => {
                format!(
                    "{} MY,{},{},{},{},{}",
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
            MoveY::Static(ref mut current_depth, ..) => *current_depth = depth,
            MoveY::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            MoveY::Static(_, start_time, _) => *start_time,
            MoveY::Dynamic(_, _, start_time, ..) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            MoveY::Static(_, end_time, _) => *end_time,
            MoveY::Dynamic(_, _, _, end_time, ..) => *end_time,
        }
    }
}

/// Creates a static `MoveY` event with the timestamp and the Y position of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let time = 0;
/// let pos_y = 0;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.movey_((time, pos_y));
/// ```
impl<T> Into<MoveY> for (i32, T)
where
    T: Into<Number>,
{
    fn into(self) -> MoveY {
        MoveY::Static(0, self.0, self.1.into())
    }
}

/// Creates a dynamic `MoveY` event with the timestamps and the Y positions of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_pos_y = 0;
/// let end_pos_y = 240;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.movey_((start_time, end_time, start_pos_y, end_pos_y));
/// ```
impl<T, U> Into<MoveY> for (i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> MoveY {
        MoveY::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            self.2.into(),
            self.3.into(),
        )
    }
}

/// Creates a dynamic `MoveY` event with the easing, the timestamps and the Y positions of the element
///
/// Example:
/// ```
/// use osb::{Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_pos_y = 0;
/// let end_pos_y = 240;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.movey_((easing, start_time, end_time, start_pos_y, end_pos_y));
/// ```
impl<T, U> Into<MoveY> for (Easing, i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> MoveY {
        MoveY::Dynamic(0, self.0, self.1, self.2, self.3.into(), self.4.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line_static() {
        let movey_event: MoveY = (0, 240).into();
        assert_eq!(movey_event.to_line(), " MY,0,0,,240");

        let mut movey_event_depth: MoveY = (0, 240).into();
        movey_event_depth.set_depth(2);
        assert_eq!(movey_event_depth.to_line(), "   MY,0,0,,240");
    }

    #[test]
    fn to_line_dynamic() {
        let movey_event: MoveY = (0, 1000, 0, 240).into();
        assert_eq!(movey_event.to_line(), " MY,0,0,1000,0,240");

        let movey_event_easing: MoveY = (Easing::QuadOut, 0, 1000, 0, 240).into();
        assert_eq!(movey_event_easing.to_line(), " MY,4,0,1000,0,240");
    }

    #[test]
    fn to_line_dynamic_float() {
        let movey_event: MoveY = (0, 1000, 0.25, 240.75).into();
        assert_eq!(movey_event.to_line(), " MY,0,0,1000,0.25,240.75");

        let movey_event_easing: MoveY = (Easing::QuadOut, 0, 1000, 0.25, 240.75).into();
        assert_eq!(movey_event_easing.to_line(), " MY,4,0,1000,0.25,240.75");
    }
}
