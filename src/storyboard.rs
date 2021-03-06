use crate::{Layer, Module};
use std::io::{self, Write};

/// What defines a storyboard
///
/// The usage of the struct `Storyboard` is a bit different from what you may be used to in other
/// languages. We'd recommend you take a look at the struct [`Module`] to understand how
/// `Storyboard`s are split in different `Module`s, improving modularity and speed.
pub struct Storyboard {
    background_modules: Vec<Module>,
    fail_modules: Vec<Module>,
    pass_modules: Vec<Module>,
    foreground_modules: Vec<Module>,
    overlay_modules: Vec<Module>,
}

fn modules_to_str(modules: &Vec<Module>) -> String {
    modules
        .iter()
        .map(|m| m.output())
        .collect::<Vec<String>>()
        .join("")
}

impl Storyboard {
    /// Initializes a `Storyboard`
    pub fn new() -> Self {
        Self {
            background_modules: vec![],
            fail_modules: vec![],
            pass_modules: vec![],
            foreground_modules: vec![],
            overlay_modules: vec![],
        }
    }

    /// Adds a [`Module`] to our `Storyboard`
    ///
    /// Usage:
    /// ```
    /// use osb::{Layer, Module, Storyboard};
    /// let mut sb = Storyboard::new();
    /// let mut your_module = Module::new(Layer::Background);
    /// sb.push(your_module);
    /// ```
    pub fn push(&mut self, module: Module) {
        match module.layer() {
            Layer::Background => self.background_modules.push(module),
            Layer::Fail => self.fail_modules.push(module),
            Layer::Pass => self.pass_modules.push(module),
            Layer::Foreground => self.foreground_modules.push(module),
            Layer::Overlay => self.overlay_modules.push(module),
        }
    }

    /// Prints our `Storyboard` to `stdout`
    ///
    /// Usage:
    /// ```
    /// use osb::Storyboard;
    /// let mut sb = Storyboard::new();
    /// sb.print().unwrap();
    /// ```
    pub fn print(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        stdout.write_all(b"[Events]\n")?;
        stdout.write_all(b"//Background and Video events\n")?;
        stdout.write_all(b"//Storyboard Layer 0 (Background)\n")?;
        stdout.write_all(modules_to_str(&self.background_modules).as_bytes())?;
        stdout.write_all(b"//Storyboard Layer 1 (Fail)\n")?;
        stdout.write_all(modules_to_str(&self.fail_modules).as_bytes())?;
        stdout.write_all(b"//Storyboard Layer 2 (Pass)\n")?;
        stdout.write_all(modules_to_str(&self.pass_modules).as_bytes())?;
        stdout.write_all(b"//Storyboard Layer 3 (Foreground)\n")?;
        stdout.write_all(modules_to_str(&self.foreground_modules).as_bytes())?;
        stdout.write_all(b"//Storyboard Layer 4 (Overlay)\n")?;
        stdout.write_all(modules_to_str(&self.overlay_modules).as_bytes())?;
        stdout.write_all(b"//Storyboard Sound Samples\n")
    }
}

use std::fmt;

impl fmt::Display for Storyboard {
    /// Formats the value using the given formatter
    ///
    /// Usage:
    /// ```
    /// use osb::Storyboard;
    /// let mut sb = Storyboard::new();
    /// println!("{}", sb);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[Events]")?;
        writeln!(f, "//Background and Video events")?;
        writeln!(f, "//Storyboard Layer 0 (Background)")?;
        write!(f, "{}", modules_to_str(&self.background_modules))?;
        writeln!(f, "//Storyboard Layer 1 (Fail)")?;
        write!(f, "{}", modules_to_str(&self.fail_modules))?;
        writeln!(f, "//Storyboard Layer 2 (Pass)")?;
        write!(f, "{}", modules_to_str(&self.pass_modules))?;
        writeln!(f, "//Storyboard Layer 3 (Foreground)")?;
        write!(f, "{}", modules_to_str(&self.foreground_modules))?;
        writeln!(f, "//Storyboard Layer 4 (Overlay)")?;
        write!(f, "{}", modules_to_str(&self.overlay_modules))?;
        write!(f, "//Storyboard Sound Samples")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Layer, Module, Storyboard};

    #[test]
    fn modules() {
        let mut sb = Storyboard::new();
        let fail_module = Module::new(Layer::Fail);
        let pass_module = Module::new(Layer::Pass);
        let foreground_module = Module::new(Layer::Foreground);
        let overlay_module = Module::new(Layer::Overlay);
        sb.push(fail_module);
        sb.push(pass_module);
        sb.push(foreground_module);
        sb.push(overlay_module);
    }
}
