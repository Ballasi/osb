// Copyright 2021 Thomas Ballasi
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::event::*;
use crate::utils::Vec2;
use crate::Layer;
use crate::Origin;

/// The struct corresponding to sprites
pub struct Sprite {
    move_events: Vec<Move>,
    current_depth: usize,
    path: String,
    pos: Vec2,
    layer: Layer,
    origin: Origin,
}

impl Sprite {
    /// Initializes a new `Sprite`
    ///
    /// See [trait implementations](#trait-implementations) to see how you can create a sprite
    pub fn new<T>(args: T) -> Self
    where
        T: Into<Sprite>,
    {
        args.into()
    }

    /// Performs the event `Move` to a `Sprite`
    ///
    /// ```
    /// use osb::{ Sprite, Easing, utils::Vec2 };
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    ///
    /// // There's a `Vec2` type you can use if you wish
    /// sprite.move_((Easing::Out, 0, 1000, Vec2::from(0, 0), Vec2::from(320, 240)));
    /// // But you're not forced to! Giving pairs of integers automatically translates to a `Vec2`
    /// sprite.move_((Easing::QuadInOut, 1000, 2000, 320, 240, 100, 100));
    /// // And of course you can use a static move too
    /// sprite.move_((3000, Vec2::from(320, 240)));
    /// ```

    pub fn move_<T>(&mut self, args: T)
    where
        T: Into<Move>,
    {
        let mut event = args.into();
        event.set_depth(self.current_depth);
        self.move_events.push(event);
    }

    /// Returns the contents of the `Sprite`
    ///
    /// **Warning**: this method is not meant to be used
    pub fn to_str(&self) -> String {
        let output = format!(
            "Sprite,{},{},\"{}\",{},{}\n{}\n",
            self.layer,
            self.origin,
            self.path,
            self.pos.x,
            self.pos.y,
            self.move_events
                .iter()
                .map(|event| event.to_line())
                .collect::<Vec<String>>()
                .join("\n")
        );
        output
    }

    /// Sets the [`Layer`] of the `Sprite`
    ///
    /// **Warning**: this method is not meant to be used
    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = layer;
    }
}

/// Creates a `Sprite` with the path of the file
///
/// Example:
/// ```
/// use osb::Sprite;
/// let path = String::from("res/sprite.png");
/// let mut sprite = Sprite::new(path);
/// ```
impl Into<Sprite> for String {
    fn into(self) -> Sprite {
        Sprite {
            move_events: Vec::<Move>::new(),
            current_depth: 0,
            path: self,
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: Origin::Centre,
        }
    }
}

/// Creates a `Sprite` with the path of the file
///
/// Example:
/// ```
/// use osb::Sprite;
/// let path = "res/sprite.png";
/// let mut sprite = Sprite::new(path);
/// ```
impl Into<Sprite> for &str {
    fn into(self) -> Sprite {
        Sprite {
            move_events: Vec::<Move>::new(),
            current_depth: 0,
            path: String::from(self),
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: Origin::Centre,
        }
    }
}

/// Creates a `Sprite` with the origin and path of the file
///
/// Example:
/// ```
/// use osb::{Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let mut sprite = Sprite::new((origin, path));
/// ```
impl Into<Sprite> for (Origin, String) {
    fn into(self) -> Sprite {
        Sprite {
            move_events: Vec::<Move>::new(),
            current_depth: 0,
            path: self.1,
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: self.0,
        }
    }
}

/// Creates a `Sprite` with the origin and the path of the file
///
/// Example:
/// ```
/// use osb::{Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let mut sprite = Sprite::new((origin, path));
/// ```
impl Into<Sprite> for (Origin, &str) {
    fn into(self) -> Sprite {
        Sprite {
            move_events: Vec::<Move>::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: self.0,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = String::from("res/sprite.png");
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((path, pos));
/// ```
impl Into<Sprite> for (String, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            move_events: Vec::<Move>::new(),
            current_depth: 0,
            path: self.0,
            pos: self.1,
            layer: Layer::Background,
            origin: Origin::Centre,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = "res/sprite.png";
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((path, pos));
/// ```
impl Into<Sprite> for (&str, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            move_events: Vec::<Move>::new(),
            current_depth: 0,
            path: String::from(self.0),
            pos: self.1,
            layer: Layer::Background,
            origin: Origin::Centre,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((origin, path, pos));
/// ```
impl Into<Sprite> for (Origin, String, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            move_events: Vec::<Move>::new(),
            current_depth: 0,
            path: self.1,
            pos: self.2,
            layer: Layer::Background,
            origin: self.0,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((origin, path, pos));
/// ```
impl Into<Sprite> for (Origin, &str, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            move_events: Vec::<Move>::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: self.2,
            layer: Layer::Background,
            origin: self.0,
        }
    }
}
