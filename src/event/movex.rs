use crate::easing::Easing;
use crate::utils::Number;
use crate::Event;

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line_static() {
        let movex_event: MoveX = (0, 320).into();
        assert_eq!(movex_event.to_line(), " MX,0,0,,320");

        let mut movex_event_depth: MoveX = (0, 320).into();
        movex_event_depth.set_depth(2);
        assert_eq!(movex_event_depth.to_line(), "   MX,0,0,,320");
    }

    #[test]
    fn to_line_dynamic() {
        let movex_event: MoveX = (0, 1000, 0, 320).into();
        assert_eq!(movex_event.to_line(), " MX,0,0,1000,0,320");

        let movex_event_easing: MoveX = (Easing::QuadOut, 0, 1000, 0, 320).into();
        assert_eq!(movex_event_easing.to_line(), " MX,4,0,1000,0,320");
    }

    #[test]
    fn to_line_dynamic_float() {
        let movex_event: MoveX = (0, 1000, 0.25, 320.75).into();
        assert_eq!(movex_event.to_line(), " MX,0,0,1000,0.25,320.75");

        let movex_event_easing: MoveX = (Easing::QuadOut, 0, 1000, 0.25, 320.75).into();
        assert_eq!(movex_event_easing.to_line(), " MX,4,0,1000,0.25,320.75");
    }
}

/// `MoveX` event
pub enum MoveX {
    Static(usize, i32, Number),
    Dynamic(usize, Easing, i32, i32, Number, Number),
}

impl Event for MoveX {
    fn to_line(&self) -> String {
        match self {
            MoveX::Static(depth, time, value) => {
                format!(
                    "{} MX,{},{},,{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    value
                )
            }
            MoveX::Dynamic(depth, easing, start_time, end_time, start_value, end_value) => {
                format!(
                    "{} MX,{},{},{},{},{}",
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
            MoveX::Static(ref mut current_depth, ..) => *current_depth = depth,
            MoveX::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            MoveX::Static(_, start_time, _) => *start_time,
            MoveX::Dynamic(_, _, start_time, ..) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            MoveX::Static(_, end_time, _) => *end_time,
            MoveX::Dynamic(_, _, _, end_time, ..) => *end_time,
        }
    }
}

/// Creates a static `MoveX` event with the timestamp and the X position of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let time = 0;
/// let pos_x = 0;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.movex_((time, pos_x));
/// ```
impl<T> Into<MoveX> for (i32, T)
where
    T: Into<Number>,
{
    fn into(self) -> MoveX {
        MoveX::Static(0, self.0, self.1.into())
    }
}

/// Creates a dynamic `MoveX` event with the timestamps and the X positions of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_pos_x = 0;
/// let end_pos_x = 320;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.movex_((start_time, end_time, start_pos_x, end_pos_x));
/// ```
impl<T, U> Into<MoveX> for (i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> MoveX {
        MoveX::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            self.2.into(),
            self.3.into(),
        )
    }
}

/// Creates a dynamic `MoveX` event with the easing, the timestamps and the X positions of the element
///
/// Example:
/// ```
/// use osb::{Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_pos_x = 0;
/// let end_pos_x = 320;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.movex_((easing, start_time, end_time, start_pos_x, end_pos_x));
/// ```
impl<T, U> Into<MoveX> for (Easing, i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> MoveX {
        MoveX::Dynamic(0, self.0, self.1, self.2, self.3.into(), self.4.into())
    }
}
