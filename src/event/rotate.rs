use crate::easing::Easing;
use crate::utils::Number;
use crate::Event;

/// `Rotate` event
pub enum Rotate {
    Static(usize, i32, Number),
    Dynamic(usize, Easing, i32, i32, Number, Number),
}

impl Event for Rotate {
    fn to_line(&self) -> String {
        match self {
            Rotate::Static(depth, time, value) => {
                format!(
                    "{} R,{},{},,{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    value
                )
            }
            Rotate::Dynamic(depth, easing, start_time, end_time, start_value, end_value) => {
                format!(
                    "{} R,{},{},{},{},{}",
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
            Rotate::Static(ref mut current_depth, ..) => *current_depth = depth,
            Rotate::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }

    fn get_start_time(&self) -> i32 {
        match self {
            Rotate::Static(_, start_time, _) => *start_time,
            Rotate::Dynamic(_, _, start_time, ..) => *start_time,
        }
    }

    fn get_end_time(&self) -> i32 {
        match self {
            Rotate::Static(_, end_time, _) => *end_time,
            Rotate::Dynamic(_, _, _, end_time, ..) => *end_time,
        }
    }
}

/// Creates a static `Rotate` event with the timestamp and the rotation of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let time = 0;
/// let rotation = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.rotate_((time, rotation));
/// ```
impl<T> Into<Rotate> for (i32, T)
where
    T: Into<Number>,
{
    fn into(self) -> Rotate {
        Rotate::Static(0, self.0, self.1.into())
    }
}

/// Creates a dynamic `Rotate` event with the timestamps and the rotations of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::Sprite;
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_rotation = 0;
/// let end_rotation = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.rotate_((start_time, end_time, start_rotation, end_rotation));
/// ```
impl<T, U> Into<Rotate> for (i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Rotate {
        Rotate::Dynamic(
            0,
            Easing::Linear,
            self.0,
            self.1,
            self.2.into(),
            self.3.into(),
        )
    }
}

/// Creates a dynamic `Rotate` event with the easing, the timestamps and the rotations of the element
///
/// Example:
/// ```
/// use osb::{Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_rotation = 0;
/// let end_rotation = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.rotate_((easing, start_time, end_time, start_rotation, end_rotation));
/// ```
impl<T, U> Into<Rotate> for (Easing, i32, i32, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Rotate {
        Rotate::Dynamic(0, self.0, self.1, self.2, self.3.into(), self.4.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line_static() {
        let rotate_event_neg: Rotate = (0, -1).into();
        assert_eq!(rotate_event_neg.to_line(), " R,0,0,,-1");

        let mut rotate_event_depth: Rotate = (0, 1).into();
        rotate_event_depth.set_depth(2);
        assert_eq!(rotate_event_depth.to_line(), "   R,0,0,,1");
    }

    #[test]
    fn to_line_dynamic() {
        let rotate_event: Rotate = (0, 1000, 0, 1).into();
        assert_eq!(rotate_event.to_line(), " R,0,0,1000,0,1");

        let rotate_event_easing: Rotate = (Easing::QuadOut, 0, 1000, 0, 1).into();
        assert_eq!(rotate_event_easing.to_line(), " R,4,0,1000,0,1");
    }

    #[test]
    fn to_line_dynamic_float() {
        let rotate_event: Rotate = (0, 1000, 0.25, 0.75).into();
        assert_eq!(rotate_event.to_line(), " R,0,0,1000,0.25,0.75");

        let rotate_event_easing: Rotate = (Easing::QuadOut, 0, 1000, 0.25, 0.75).into();
        assert_eq!(rotate_event_easing.to_line(), " R,4,0,1000,0.25,0.75");
    }
}
