use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

// Pixels per coordinate unit.
const BLOCK_SIZE: f64 = 25.0;

// Draws a block.
pub fn draw_block(color: Color, x: i32, y: i32, ctx: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        ctx.transform,
        g,
    );
}

// Draws a rectangle.
pub fn draw_rect(color: Color, x: i32, y: i32, width: i32, height: i32, ctx: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, (width as f64) * BLOCK_SIZE, (height as f64) * BLOCK_SIZE],
        ctx.transform,
        g,
    );
}

// Converts game coordinate units to pixels.
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}