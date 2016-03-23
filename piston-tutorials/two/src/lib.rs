extern crate piston_window;
extern crate graphics;

use piston_window::*;


#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

pub fn two() {
    let width = 640f64;
    let height = 50f64;

    let window: PistonWindow = WindowSettings::new("Hello Piston!", [width as u32, height as u32])
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap();

    let mut ball_x_pos: f64 = 0.0;

    let ball_size = 50.0;
    let box_size = 50.0;

    let mut box_x_pos: f64 = (width / 2f64) - box_size / 2f64;

    let mut ball_x_direction = Direction::Right;

    let mut over = false;

    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);

            ellipse([1.0, 0.0, 0.0, 1.0],
                    [ball_x_pos, 0.0, ball_size, ball_size],
                    c.transform,
                    g);

            let mut color = match over {
                true => [0.0, 0.0, 1.0, 1.0],
                false => [1.0, 0.0, 0.0, 1.0],
            };

            rectangle(color, [box_x_pos, 0.0, box_size, box_size], c.transform, g);
        });

        ball_x_direction = match ball_x_direction {
            Direction::Right if (ball_x_pos >= width - ball_size) => Direction::Left,
            Direction::Left if (ball_x_pos <= 0f64) => Direction::Right,
            _ => ball_x_direction,
        };

        match ball_x_direction {
            Direction::Right => ball_x_pos += 0.5f64,
            Direction::Left => ball_x_pos -= 0.5f64,
        }

        // It's over if ball position + ball_size is greater than box position and lower than
        // box position + box_size

        over = match ball_x_pos {
            pos if (pos + ball_size) >= box_x_pos && pos <= (box_x_pos + box_size) => true,
            _ => false,
        }
    }
}
