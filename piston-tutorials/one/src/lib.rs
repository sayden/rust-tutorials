extern crate piston_window;
extern crate graphics;

use piston_window::*;


#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn one() {
    let width = 640f64;
    let height = 480f64;

    let window: PistonWindow = WindowSettings::new("Hello Piston!", [width as u32, height as u32])
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap();

    let mut y_count: f64 = 0.0;
    let mut x_count: f64 = 0.0;

    let ball_size = 50.0;

    let mut x_direction = Direction::Right;
    let mut y_direction = Direction::Down;

    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);

            ellipse([1.0, 0.0, 0.0, 1.0],
                    [x_count, y_count, ball_size, ball_size],
                    c.transform,
                    g);
        });

        y_direction = match y_direction {
            Direction::Down if (y_count >= height - ball_size) => Direction::Up,
            Direction::Up if (y_count <= 0f64) => Direction::Down,
            _ => y_direction,
        };

        match y_direction {
            Direction::Down => y_count += 0.5f64,
            Direction::Up => y_count -= 0.5f64,
            _ => y_count += 0f64,
        }

        x_direction = match x_direction {
            Direction::Right if (x_count >= width - ball_size) => Direction::Left,
            Direction::Left if (x_count <= 0f64) => Direction::Right,
            _ => x_direction,
        };

        match x_direction {
            Direction::Right => x_count += 0.5f64,
            Direction::Left => x_count -= 0.5f64,
            _ => x_count += 0f64,
        }
    }
}
