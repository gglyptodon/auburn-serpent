use bracket_lib::prelude::*;
use serpent::serpent::misc;
embedded_resource!(TILE_FONT, "../resources/tiles4.png");

fn main() -> BResult<()> {
    link_resource!(TILE_FONT, "resources/tiles4.png");
    let context = BTermBuilder::new()
        .with_font("tiles4.png", 32, 32)
        .with_simple_console(serpent::WIDTH, serpent::HEIGHT, "tiles4.png")
        .with_fancy_console(serpent::WIDTH, serpent::HEIGHT, "tiles4.png")
        .with_title("Serpent")
        .with_fps_cap(30.0)
        //.with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, misc::State::new())
}

#[test]
fn test_imports_player() {
    serpent::serpent::player::hello_player();
}
