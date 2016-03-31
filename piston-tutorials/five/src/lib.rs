extern crate piston_window;
extern crate graphics;

use piston_window::*;


#[derive(PartialEq, Clone)]
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

    fn draw(&self, c: Context, g: &mut G2d) {
        ellipse([1.0, 0.0, 0.0, 1.0],
                [self.x_pos, self.y_pos, self.diameter, self.diameter],
                c.transform,
                g);
    }

    fn check_new_position(&mut self, game: &Game) {
        let cur = self.x_direction.clone();
        self.x_direction = match self.x_direction {
            Direction::Right if (self.x_pos >= game.width - self.diameter) => Direction::Left,
            Direction::Left if (self.x_pos <= 0f64) => Direction::Right,
            _ => cur,
        };

        let cur = self.y_direction.clone();
        self.y_direction = match self.y_direction {
            Direction::Down if (self.y_pos >= game.height - self.diameter) => Direction::Up,
            Direction::Up if (self.y_pos <= 0f64) => Direction::Down,
            _ => cur,
        };

        match self.x_direction {
            Direction::Right => self.x_pos += 0.5f64,
            Direction::Left => self.x_pos -= 0.5f64,
            _ => self.x_pos += 0.0f64,
        }

        match self.y_direction {
            Direction::Down => self.y_pos += 0.5f64,
            Direction::Up => self.y_pos -= 0.5f64,
            _ => self.y_pos -= 0.0f64,
        }
    }
}

pub struct GameBox {
    x_pos: f64,
    y_pos: f64,
    size: f64,
}

impl GameBox {
    pub fn new(g: &Game, size: f64) -> Self {
        GameBox {
            x_pos: (g.width / 2f64) - size / 2f64,
            y_pos: (g.height / 2f64) - size / 2f64,
            size: size,
        }
    }

    fn draw(&self, c: Context, g: &mut G2d, ball: &Ball) {
        // It's over if ball position + ball_size is greater than box position and lower than
        // box position + box_size
        let over = match (ball.x_pos, ball.y_pos) {
            (pos_x, pos_y) if (pos_x + ball.diameter) >= self.x_pos &&
                              pos_x <= (self.x_pos + self.size) &&
                              (pos_y + ball.diameter) >= self.y_pos &&
                              pos_y <= (self.y_pos + self.size) => true,
            _ => false,
        };

        let color = match over {
            true => [0.0, 0.0, 1.0, 1.0],
            false => [1.0, 0.0, 0.0, 1.0],
        };

        rectangle(color,
                  [self.x_pos, self.y_pos, self.size, self.size],
                  c.transform,
                  g);
    }

    fn check_events(&mut self, e: &PistonWindow) {
        // Check keypresses
        match e.clone().event {
            Some(Event::Input(inp)) => {
                match inp {
                    Input::Press(but) => {
                        match but {
                            Button::Keyboard(Key::Up) => self.y_pos -= 5f64,
                            Button::Keyboard(Key::Down) => self.y_pos += 5f64,
                            Button::Keyboard(Key::Right) => self.x_pos += 5f64,
                            Button::Keyboard(Key::Left) => self.x_pos -= 5f64,
                            _ => self.x_pos += 0f64,
                        };
                    }
                    _ => self.x_pos += 0f64,
                }
            }
            _ => (),
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
    let mut game_box = GameBox::new(&game, 100 as f64);

    for e in window {

        ball.check_new_position(&game);

        // Used to respond to user input and move the box
        game_box.check_events(&e);

        // Draw the resulting image
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);

            ball.draw(c, g);

            // Draw the box, we pass it the ball to check collisions against box and
            // act accordingly
            game_box.draw(c, g, &ball)
        });
    }
}
