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
    x_direction: Direction,
    y_direction: Direction,
}

impl Ball {
    pub fn new(start_x_pos: f64, start_y_pos: f64, diameter: f64) -> Self {
        Ball {
            x_pos: start_x_pos,
            y_pos: start_y_pos,
            diameter: diameter,
            x_direction: Direction::Right,
            y_direction: Direction::Down,
        }
    }
}

pub struct Game_Box {
    x_pos: f64,
    y_pos: f64,
    size: f64,
}

impl Game_Box {
    pub fn new(g: &Game, size: f64) -> Self {
        Game_Box {
            x_pos: (g.width / 2f64) - size / 2f64,
            y_pos: (g.height / 2f64) - size / 2f64,
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


    let mut ball = Ball::new(0.0, 0.0, 50.0);

    let game_box = Game_Box::new(&game, 100 as f64);

    // let mut box_x_pos: f64 = (game.width / 2f64) - game_box.size / 2f64;
    // let mut box_y_pos: f64 = (game.height / 2f64) - game_box.size / 2f64;

    let mut over: bool;

    for e in window {

        ball.x_direction = match ball.x_direction {
            Direction::Right if (ball.x_pos >= game.width - ball.diameter) => Direction::Left,
            Direction::Left if (ball.x_pos <= 0f64) => Direction::Right,
            _ => ball.x_direction,
        };

        ball.y_direction = match ball.y_direction {
            Direction::Down if (ball.y_pos >= game.height - ball.diameter) => Direction::Up,
            Direction::Up if (ball.y_pos <= 0f64) => Direction::Down,
            _ => ball.y_direction,
        };

        match ball.x_direction {
            Direction::Right => ball.x_pos += 0.5f64,
            Direction::Left => ball.x_pos -= 0.5f64,
            _ => ball.x_pos += 0.0f64,
        }

        match ball.y_direction {
            Direction::Down => ball.y_pos += 0.5f64,
            Direction::Up => ball.y_pos -= 0.5f64,
            _ => ball.y_pos -= 0.0f64,
        }

        // It's over if ball position + ball_size is greater than box position and lower than
        // box position + box_size
        over = match (ball.x_pos, ball.y_pos) {
            (pos_x, pos_y) if (pos_x + ball.diameter) >= game_box.x_pos &&
                              pos_x <= (game_box.x_pos + game_box.size) &&
                              (pos_y + ball.diameter) >= box_y_pos &&
                              pos_y <= (box_y_pos + game_box.size) => true,
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
                            Button::Keyboard(Key::Right) => game_box.x_pos += 5f64,
                            Button::Keyboard(Key::Left) => game_box.x_pos -= 5f64,
                            _ => game_box.x_pos += 0f64,
                        };
                    }
                    _ => game_box.x_pos += 0f64,
                }
            }
            _ => (),
        }

        // Draw the resulting image
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);

            ellipse([1.0, 0.0, 0.0, 1.0],
                    [ball.x_pos, ball.y_pos, ball.diameter, ball.diameter],
                    c.transform,
                    g);

            let color = match over {
                true => [0.0, 0.0, 1.0, 1.0],
                false => [1.0, 0.0, 0.0, 1.0],
            };

            rectangle(color,
                      [game_box.x_pos, box_y_pos, game_box.size, game_box.size],
                      c.transform,
                      g);
        });
    }
}
