use std::fmt;

/// `Layer`s as defined in the [official osu! specifications](https://osu.ppy.sh/wiki/en/Storyboard_Scripting/General_Rules#layers)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layer {
    /// Default and preferred layer
    Background,
    /// Storyboard elements will appear when the player is on the verge of
    /// losing
    ///
    /// Appears only when `Pass` is not
    Fail,
    /// Storyboard elements will appear when the player is winning
    ///
    /// Appears only when `Fail` is not
    Pass,
    /// Default layer
    Foreground,
    /// Storyboard elements will appear above the playfield
    ///
    /// Poorly documented but supported in osu!, trust me, it works
    Overlay,
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
                Layer::Overlay => "Overlay",
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
        assert_eq!(format!("{}", Layer::Overlay), "Overlay");
    }
}
