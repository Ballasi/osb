// Copyright 2021 Thomas Ballasi
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{Layer, Sprite};

/// A component of a `Storyboard`
///
/// Storyboards, in `osb`, are split into different components called `Module`s. This allows the
/// storyboarder to have the good practice of splitting a `Storyboard` into different parts
/// efficiently.
///
/// In terms of scalabitily, this allows anyone to create, share and import `Module`s of any
/// kind. Everyone can publish their own `Module` to crates.io and anyone can use them.
///
/// This forces the storyboarder to initialize and push `Module`s properly onto their
/// `Storyboard`s. It is recommended to create a function for each new `Module` or create a file
/// of its own depending on the complexity of your module.
///
/// A `Module` is tied to a [`Layer`] and cannot be changed.
pub struct Module {
    layer: Layer,
    sprites: Vec<Sprite>,
}

impl Module {
    /// Initializes a new `Module`
    pub fn new(layer: Layer) -> Self {
        Self {
            layer,
            sprites: vec![],
        }
    }

    /// Adds a [`Sprite`] to a `Module`
    ///
    /// Usage:
    /// ```
    /// use osb::{Layer, Module, Sprite};
    /// let mut module = Module::new(Layer::Background);
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// module.push(sprite);
    /// ```
    pub fn push(&mut self, mut sprite: Sprite) {
        sprite.set_layer(self.layer);
        self.sprites.push(sprite);
    }

    /// Returns the contents of the `Module`
    ///
    /// **Warning**: this method is not meant to be used
    pub fn output(&self) -> String {
        self.sprites
            .iter()
            .map(|spr| spr.to_str())
            .collect::<Vec<String>>()
            .join("")
    }

    /// Returns the layer of the `Module`
    ///
    /// Example:
    /// ```
    /// use osb::{Layer, Module};
    /// let mut module = Module::new(Layer::Background);
    /// assert_eq!(module.layer(), Layer::Background);
    /// ```
    pub fn layer(&self) -> Layer {
        self.layer
    }
}
