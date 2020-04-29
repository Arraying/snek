
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::game::Coordinate;
use crate::draw::draw_block;

const SNAKE_COLOR: Color = [128.0 / 255.0, 211.0 /255.0, 76.0 / 255.0, 1.0];
const SNAKE_COLOR_HEAD: Color = [46.0 / 255.0, 136.0 / 255.0, 57.0 / 255.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

// The direction the snake is travelling in.
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// The snake representation.
pub struct Snake {
    direction: Direction,
    body: LinkedList<Coordinate>,
    tail: Option<Coordinate>,
}

impl Snake {

    // Generate a new snake.
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Coordinate> = LinkedList::new();
        for i in 0..=2 { // Programatically create a snake of 3 length.
            body.push_front(Coordinate {
                x: x + i,
                y
            });
        }
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // Draw the snake by drawing each block.
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for (index, block) in self.body.iter().enumerate() {
            let color = if index == 0 {
                SNAKE_COLOR_HEAD
            } else {
                SNAKE_COLOR
            };
            draw_block(color, block.x, block.y, ctx, g);
        }
    }

    // Gets the coordinates of the head.
    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    // Gets the head direction (i.e. the direction the snake is travelling in).
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // Gets the position the head will be in at the next update.
    // This returns the coordinate of a position in which the snake is not in currently.
    pub fn head_next(&self, dir: Option<Direction>) -> (i32, i32) {
        let current = self.head_position();
        let mut direction = self.direction;
        match dir {
            Some(d) => direction = d,
            None => {}
        }
        match direction {
            Direction::Up => (current.0, current.1 - 1),
            Direction::Down => (current.0, current.1 + 1), 
            Direction::Left => (current.0 - 1, current.1),
            Direction::Right => (current.0 + 1, current.1)
        }
    }

    // Restores the old tail which is not rendered, effectively growing the snake.
    pub fn tail_restore(&mut self) {
        match self.tail.clone() {
            Some(tail) => self.body.push_back(tail),
            None => {}
        }
    }

    // Checks if the tail overlaps with itself.
    pub fn tail_overlap(&self, x: i32, y: i32) -> bool {
        for (index, block) in self.body.iter().enumerate() { // Enumerate to have a counter: index.
            if x == block.x && y == block.y {
                return true
            }
            if index + 1 == self.body.len() - 1 { // Ignore the head.
                break
            }
        }
        false
    }

    // Moves the snake forward.
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d, // Update the direction if applicable.
            None => {},
        }

        let pos = self.head_position();
        // Create a new position for the head.
        let new_block = match self.direction {
            Direction::Up => {
                Coordinate {
                    x: pos.0,
                    y: pos.1 - 1,
                }
            },
            Direction::Down => {
                Coordinate {
                    x: pos.0,
                    y: pos.1 + 1,
                }
            },
            Direction::Left => {
                Coordinate {
                    x: pos.0 - 1,
                    y: pos.1,
                }
            },
            Direction::Right => {
                Coordinate {
                    x: pos.0 + 1,
                    y: pos.1
                }
            }
        };
        self.body.push_front(new_block); // Head goes to the top of the linked list.

        // Remove the last body block and make it into the tail, which can then be rendered when the snake grows. 
        // This can use unwrap safely because there will always be at least one head in the body list.
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

}