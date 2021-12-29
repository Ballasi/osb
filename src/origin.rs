use std::fmt;

/// `Origin`s as defined in the [official osu! specifications](https://osu.ppy.sh/wiki/en/Storyboard_Scripting/Objects)
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Origin {
    TopLeft,
    TopCentre,
    TopRight,
    CentreLeft,
    Centre,
    CentreRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Origin::TopLeft => "TopLeft",
                Origin::TopCentre => "TopCentre",
                Origin::TopRight => "TopRight",
                Origin::CentreLeft => "CentreLeft",
                Origin::Centre => "Centre",
                Origin::CentreRight => "CentreRight",
                Origin::BottomLeft => "BottomLeft",
                Origin::BottomCentre => "BottomCentre",
                Origin::BottomRight => "BottomRight",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Origin;

    #[test]
    fn origin() {
        assert_eq!(format!("{}", Origin::TopLeft), "TopLeft");
        assert_eq!(format!("{}", Origin::TopCentre), "TopCentre");
        assert_eq!(format!("{}", Origin::TopRight), "TopRight");
        assert_eq!(format!("{}", Origin::CentreLeft), "CentreLeft");
        assert_eq!(format!("{}", Origin::Centre), "Centre");
        assert_eq!(format!("{}", Origin::CentreRight), "CentreRight");
        assert_eq!(format!("{}", Origin::BottomLeft), "BottomLeft");
        assert_eq!(format!("{}", Origin::BottomCentre), "BottomCentre");
        assert_eq!(format!("{}", Origin::BottomRight), "BottomRight");
    }
}
