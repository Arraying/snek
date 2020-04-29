extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;

// The backdrop colour.
const BACK_COLOR: Color = [1.0, 1.0, 153.0 / 255.0, 1.0];

fn main() {
    // Dimensions of the window in coordinate units.
    let (w, h) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("Snek", [draw::to_coord(w), draw::to_coord(h)])
        .exit_on_esc(true)
        .build()
        .expect("could not create window");
    
    // Creates a new game.
    let mut game = Game::new(w, h);

    // Event handler.
    while let Some(event) = window.next() {

        // On key press.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Render and update game.
        window.draw_2d(&event, |ctx, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&ctx, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
