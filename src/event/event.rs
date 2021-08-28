// Copyright 2021 Thomas Ballasi
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Trait defining `Event`s
pub trait Event {
    fn to_line(&self) -> String;
    fn set_depth(&mut self, depth: usize);
    fn get_start_time(&self) -> i32;
    fn get_end_time(&self) -> i32;
}
