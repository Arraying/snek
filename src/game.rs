use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rect};

// Default game variables that get applied on creation and restart.
const GAME_SNAKE_X: i32 = 2;
const GAME_SNAKE_Y: i32 = 2;
const GAME_WAIT: f64 = 0.0;
const GAME_FOOD_X: i32 = 6;
const GAME_FOOD_Y: i32 = 4;

const MOVING_PERIOD: f64 = 0.075; // How fast the snake will move.
const RESTART_TIME: f64 = 1.0; // Delay (in seconds) before a restart.

// Colours
const FOOD_COLOR: Color = [1.0, 80.0 / 255.0, 80.0 / 255.0, 1.0];
const BORDER_COLOR: Color = [30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0, 1.0];
const GAME_OVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

// The game.
pub struct Game {
    snake: Snake,

    // The location of the food as an option.
    food: Option<Coordinate>,
    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

// Represents a point using game coordinate units.
// This could be a block, or just a reference point.
#[derive(Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Game {

    // Creates a new game with the specified width and height.
    // This height is in coordinate units, not pixels.
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(GAME_SNAKE_X, GAME_SNAKE_Y),
            waiting_time: GAME_WAIT,
            food: Some(Coordinate {
                x: GAME_FOOD_X,
                y: GAME_FOOD_Y
            }),
            width,
            height,
            game_over: false,
        }
    }

    // Restarts the game.
    fn restart(&mut self) {
        self.snake = Snake::new(GAME_SNAKE_X, GAME_SNAKE_Y);
        self.waiting_time = GAME_WAIT;
        self.food = Some(Coordinate {
            x: GAME_FOOD_X,
            y: GAME_FOOD_Y
        });
        self.game_over = false;
    }

    // Draws the actual game. This will render the "arena", food and snake.    
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.snake.draw(ctx, g);

        // Draw food if it exists.
        match &self.food {
            Some(f) => draw_block(FOOD_COLOR, f.x, f.y, ctx, g),
            _ => {}
        }

        draw_rect(BORDER_COLOR, 0, 0, self.width, 1, ctx, g);
        draw_rect(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g);
        draw_rect(BORDER_COLOR, 0, 0, 1, self.height, ctx, g);
        draw_rect(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g);

        // Draw game over overlay.
        if self.game_over {
            draw_rect(GAME_OVER_COLOR, 0, 0, self.width, self.height, ctx, g);
        }
    }

    // Run at each tick of the game.
    pub fn update(&mut self, delta_time: f64) {
        
        // Add to the idle time.
        self.waiting_time += delta_time;

        // Check if the game has been over long enough to restart.
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if self.food.is_none() {
            self.food_add();
        }

        // Tick that updates the snake.
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // Actually update the snake.
    // This will check if conditions are met.
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    // Event handler for keypress.
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        // Only match arrow keys to control snake.
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None
        };

        // Evaluate direction if present.
        dir.iter().for_each(|direction| {
            // Make sure it's not the opposite direction, that is not allowed in snake.
            // Additionally, it cannot be the same direction either. If it is, then the snake will go super
            // fast and appear slightly glitchy. 
            // Some implementations may see this as a feature, but it is unwanted here.
            let head_direction = self.snake.head_direction();
            if direction != &head_direction.opposite() && direction != &head_direction {
                self.update_snake(dir);
            }
        });

    }

    // Checks if the snake is eating.
    fn check_eating(&mut self) {
        let head = self.snake.head_position();
        match &self.food {
            // If food exists at the current position.
            Some(f) if f.x == head.0 && f.y == head.1 => {
                self.food = None; // Remove food.
                self.snake.tail_restore(); // Add another block to the tail.
            },
            _ => {}
        }
    }

    // Checks if the snake is still alive.
    fn check_alive(&self, dir: Option<Direction>) -> bool {
        let (x, y) = self.snake.head_next(dir); // Get the next position of the head.
        if self.snake.tail_overlap(x, y) {
            return false; // When there is overlap, kill the snake.
        }

        // Check to see if the snake is within the arena.
        x > 0 && y > 0 && x < self.width - 1 && y < self.height - 1
    }

    // Spawns food.
    fn food_add(&mut self) {
        let mut rng = thread_rng();

        let (mut x, mut y): (i32, i32) = self.rng_gen_range(&mut rng);
        // Make sure food does not spawn in the snake.
        while self.snake.tail_overlap(x, y) {
            let (imm_x, imm_y) = self.rng_gen_range(&mut rng);
            x = imm_x;
            y = imm_y;
        }

        self.food = Some(Coordinate {
            x,
            y,
        });
    }

    // Generates a random coordinate number within the arena size.
    fn rng_gen_range(&self, rng: &mut impl Rng) -> (i32, i32) {
        (rng.gen_range(1, self.width -1), rng.gen_range(1, self.height - 1))
    }

}