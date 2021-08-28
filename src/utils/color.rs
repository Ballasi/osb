#[cfg(test)]
mod tests {
    use crate::utils::Color;

    #[test]
    fn out_of_range() {
        assert_eq!(Color::from(-1, -200, -42), Color::black());
        assert_eq!(Color::from(300, 300, 300), Color::white());
    }
}

/// A color type
///
/// Contains an `r`, `g` and `b` value that ranges between 0 and 255
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl Color {
    /// Allows you to create a `Color`
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// let my_color = Color::from(42, 42, 42);
    /// ```
    pub fn from(r: i32, g: i32, b: i32) -> Self {
        let (mut r, mut g, mut b) = (r, g, b);
        if r < 0 {
            r = 0;
        } else if r > 255 {
            r = 255;
        }

        if g < 0 {
            g = 0;
        } else if g > 255 {
            g = 255;
        }

        if b < 0 {
            b = 0;
        } else if b > 255 {
            b = 255;
        }

        Self { r, g, b }
    }

    /// Returns the red value of a `Color`
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// assert_eq!(Color::red().r(), 255);
    /// ```
    pub fn r(&self) -> i32 {
        self.r
    }

    /// Returns the green value of a `Color`
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// assert_eq!(Color::green().g(), 255);
    /// ```
    pub fn g(&self) -> i32 {
        self.g
    }

    /// Returns the blue value of a `Color`
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// assert_eq!(Color::blue().b(), 255);
    /// ```
    pub fn b(&self) -> i32 {
        self.b
    }

    /// Returns a black color
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// assert_eq!(Color::black(), Color::from(0, 0, 0));
    /// ```
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    /// Returns a red color
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// assert_eq!(Color::red(), Color::from(255, 0, 0));
    /// ```
    pub fn red() -> Self {
        Self { r: 255, g: 0, b: 0 }
    }

    /// Returns a green color
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// assert_eq!(Color::green(), Color::from(0, 255, 0));
    /// ```
    pub fn green() -> Self {
        Self { r: 0, g: 255, b: 0 }
    }

    /// Returns a blue color
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// assert_eq!(Color::blue(), Color::from(0, 0, 255));
    /// ```
    pub fn blue() -> Self {
        Self { r: 0, g: 0, b: 255 }
    }

    /// Returns a white color
    ///
    /// Example:
    /// ```
    /// use osb::utils::Color;
    /// assert_eq!(Color::white(), Color::from(255, 255, 255));
    /// ```
    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl Into<Color> for (i32, i32, i32) {
    fn into(self) -> Color {
        Color::from(self.0, self.1, self.2)
    }
}
