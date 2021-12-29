use std::fmt;

/// `Layer`s as defined in the [official osu! specifications](https://osu.ppy.sh/wiki/en/Storyboard_Scripting/General_Rules#layers)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layer {
    Background,
    Fail,
    Pass,
    Foreground,
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Layer::Background => "Background",
                Layer::Fail => "Fail",
                Layer::Pass => "Pass",
                Layer::Foreground => "Foreground",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Layer;

    #[test]
    fn origin() {
        assert_eq!(format!("{}", Layer::Background), "Background");
        assert_eq!(format!("{}", Layer::Fail), "Fail");
        assert_eq!(format!("{}", Layer::Pass), "Pass");
        assert_eq!(format!("{}", Layer::Foreground), "Foreground");
    }
}
