// Copyright 2021 Thomas Ballasi
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "lib"]
#![crate_name = "osb"]

/*!
 * An easy-to-use library to create, read, parse and modify .osb storyboard files.
 *
 * The **osb** library focuses on ease, scalability and security.
 *
 * # Example
 *
 * ```
 * use osb::{Layer, Module, Sprite, Storyboard};
 *
 * fn module() -> Module {
 *     let mut module = Module::new(Layer::Background);
 *
 *     let mut sprite = Sprite::new("res/sprite.png");
 *     sprite.move_((0, 320, 240));
 *     module.push(sprite);
 *
 *     module
 * }
 *
 * fn main() -> std::io::Result<()> {
 *     let mut sb = Storyboard::new();
 *     sb.push(module());
 *     sb.print()
 * }
 * ```
 *
 * # Warning
 *
 * This crate is meant to be used by users with experience with the `.osb` file format, even
 * though the following documentation links to various sources to learn about some concepts of
 * storyboarding.
 *
 * If you are new to storyboarding, we'd recommend you to develop knowledge on this field first.
 * If you have no idea where to start, a great source of knowledge is PoNo's
 * [osbx Wiki](https://wiki.osbx.org/storyboard/osb).
 */

/// All of the storyboard events, `Move`, `Scale`, ... and the trait `Event` defining them
pub mod event;
pub use event::Event;

/// The utils, everything we need in order to make `osb` work
pub mod utils;

mod easing;
pub use easing::*;

mod layer;
pub use layer::*;

mod origin;
pub use origin::*;

mod storyboard;
pub use storyboard::*;

mod visuals;
pub use visuals::*;

mod module;
pub use module::*;

