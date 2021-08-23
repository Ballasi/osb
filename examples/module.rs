use osb::{Layer, Module, Sprite, Storyboard};

fn module() -> Module {
    let mut module = Module::new(Layer::Background);

    let mut sprite = Sprite::new("res/sprite.png");
    sprite.move_((0, 320, 240));
    module.push(sprite);

    module
}

fn main() -> std::io::Result<()> {
    let mut sb = Storyboard::new();
    sb.push(module());
    sb.print()
}
