use notan::{draw::*, prelude::*};

mod state;
#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .set_size(1080, 1920)
        .set_fullscreen(true);
    notan::init_with(setup)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    //
}
