extern crate piston_window;
extern crate graphics;

use piston_window::*;


#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

pub struct Ball {
    x_pos: f64,
    y_pos: f64,
    diameter: f64,
}

pub struct Box {
    x_pos: f64,
    y_pos: f64,
    size: u32,
}

impl Box {
    pub fn new(g: &Game, size: u32) -> Self {
        Box {
            x_pos: (g.width / 2f64) - size as f64 / 2f64,
            y_pos: (g.height / 2f64) - size as f64 / 2f64,
            size: size,
        }
    }
}

pub struct Game {
    width: f64,
    height: f64,
}

impl Game {
    pub fn new(w: f64, h: f64) -> Self {
        Game {
            width: w,
            height: h,
        }
    }
}

pub fn five() {
    let game = Game::new(640.0, 480.0);

    let window: PistonWindow = WindowSettings::new("Hello Piston!",
                                                   [game.width as u32, game.height as u32])
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap();

    let mut ball_x_pos: f64 = 0.0;
    let mut ball_y_pos: f64 = 0.0;

    let ball_size = 50.0;

    let game_box = Box::new(&game, 100);

    let mut box_x_pos: f64 = (game.width / 2f64) - game_box.size as f64 / 2f64;
    let mut box_y_pos: f64 = (game.height / 2f64) - game_box.size as f64 / 2f64;

    let mut ball_x_direction = Direction::Right;
    let mut ball_y_direction = Direction::Down;

    let mut over: bool;

    for e in window {

        ball_x_direction = match ball_x_direction {
            Direction::Right if (ball_x_pos >= game.width - ball_size) => Direction::Left,
            Direction::Left if (ball_x_pos <= 0f64) => Direction::Right,
            _ => ball_x_direction,
        };

        ball_y_direction = match ball_y_direction {
            Direction::Down if (ball_y_pos >= game.height - ball_size) => Direction::Up,
            Direction::Up if (ball_y_pos <= 0f64) => Direction::Down,
            _ => ball_y_direction,
        };

        match ball_x_direction {
            Direction::Right => ball_x_pos += 0.5f64,
            Direction::Left => ball_x_pos -= 0.5f64,
            _ => ball_x_pos += 0.0f64,
        }

        match ball_y_direction {
            Direction::Down => ball_y_pos += 0.5f64,
            Direction::Up => ball_y_pos -= 0.5f64,
            _ => ball_y_pos -= 0.0f64,
        }

        // It's over if ball position + ball_size is greater than box position and lower than
        // box position + box_size
        over = match (ball_x_pos, ball_y_pos) {
            (pos_x, pos_y) if (pos_x + ball_size) >= box_x_pos &&
                              pos_x <= (box_x_pos + game_box.size) &&
                              (pos_y + ball_size) >= box_y_pos &&
                              pos_y <= (box_y_pos) + game_box.size => true,
            _ => false,
        };

        let key_press: Direction;

        // Check keypresses
        match e.clone().event {
            Some(Event::Input(inp)) => {
                match inp {
                    Input::Press(but) => {
                        match but {
                            Button::Keyboard(Key::Up) => box_y_pos -= 5f64,
                            Button::Keyboard(Key::Down) => box_y_pos += 5f64,
                            Button::Keyboard(Key::Right) => box_x_pos += 5f64,
                            Button::Keyboard(Key::Left) => box_x_pos -= 5f64,
                            _ => box_x_pos += 0f64,
                        };
                    }
                    _ => box_x_pos += 0f64,
                }
            }
            _ => (),
        }

        // Draw the resulting image
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);

            ellipse([1.0, 0.0, 0.0, 1.0],
                    [ball_x_pos, ball_y_pos, ball_size, ball_size],
                    c.transform,
                    g);

            let color = match over {
                true => [0.0, 0.0, 1.0, 1.0],
                false => [1.0, 0.0, 0.0, 1.0],
            };

            rectangle(color,
                      [box_x_pos, box_y_pos, game_box.size, game_box.size],
                      c.transform,
                      g);
        });
    }
}
