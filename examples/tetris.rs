extern crate rust_3d;
extern crate nalgebra;
extern crate rand;

use nalgebra::DMatrix;
use rust_3d::cgmath::Deg;
use rust_3d::cgmath::Matrix;
use rust_3d::controller::Controller;
use rust_3d::glium::glutin::ElementState;
use rust_3d::glium::glutin::Event;
use rust_3d::glium::glutin::PollEventsIterator;
use rust_3d::prelude::Coord3d;
use rust_3d::prelude::Trans3d;
use rust_3d::prelude::Trans4d;
use rust_3d::render::RenderObject;
use rust_3d::render::RenderOptions;
use rust_3d::solids;
use rust_3d::viewport;
use std::time::Instant;

fn main() {
    let mut tetris = Tetris::new(10, 30);

    let blocks = RenderObject {
        geometry: solids::cube(Coord3d::new(1.0, 0.0, 0.7)),
        render_options: GameElement::Falling,
    };
    let border = RenderObject {
        geometry: solids::cube(Coord3d::new(0.5, 0.5, 0.5)),
        render_options: GameElement::Border,
    };
    let solid = RenderObject {
        geometry: solids::cube(Coord3d::new(0.0, 0.7, 1.0)),
        render_options: GameElement::Solid,
    };

    viewport::render(&[blocks, border, solid], &mut tetris);
}


struct Tetromino {
    geometry: [[(i8, i8); 4]; 4],
    orientation: u8,
    x: isize,
    y: isize,
    color: Coord3d,
}

impl Tetromino {
    fn pick_any() -> Self {
        let random_choice = rand::random::<u64>() % 2;
        match random_choice {
            0 => Self::new_t(),
            1 => Self::new_s(),
            _ => unreachable!(),
        }
    }

    fn new_t() -> Tetromino {
        Tetromino {
            geometry: [[(0, -1), (-1, -2), (0, -2), (1, -2)],
                       [(-1, 0), (-1, -1), (0, -1), (-1, -2)],
                       [(-1, 0), (0, 0), (1, 0), (0, -1)],
                       [(1, 0), (0, -1), (1, -1), (1, -2)]],
            orientation: 0,
            x: 0,
            y: 0,
            color: Coord3d::new(1.0, 0.0, 0.7),
        }
    }

    fn new_s() -> Tetromino {
        Tetromino {
            geometry: [[(-1, -1), (0, -1), (0, -2), (1, -2)],
                       [(1, 0), (0, -1), (1, -1), (0, -2)],
                       [(-1, -1), (0, -1), (0, -2), (1, -2)],
                       [(1, 0), (0, -1), (1, -1), (0, -2)]],
            orientation: 0,
            x: 0,
            y: 0,
            color: Coord3d::new(0.0, 0.7, 1.0),
        }
    }

    fn moved_by(&self, move_direction: MoveDirection) -> Self {
        match move_direction {
            MoveDirection::Right => Tetromino { x: self.x + 1, ..*self },
            MoveDirection::Left => Tetromino { x: self.x - 1, ..*self },
            MoveDirection::Down => Tetromino { y: self.y - 1, ..*self },
        }
    }

    fn rotated(&self) -> Self {
        Tetromino { orientation: (self.orientation + 1) % 4, ..*self }
    }

    fn coords(&self) -> Vec<(isize, isize)> {
        self.geometry[self.orientation as usize].iter().map(|&(x, y)| (x as isize + self.x, y as isize + self.y)).collect()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum MoveDirection {
    Left,
    Right,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BoardCell {
    transform: Trans4d,
    color: Coord3d,
}

impl BoardCell {
    fn new(x: isize, y: isize, color: Coord3d) -> Self {
        BoardCell {
            transform: place_at(x, y),
            color: color,
        }
    }
}

struct Tetris {
    game_start: Instant,
    last_drop: Instant,
    board: DMatrix<Option<BoardCell>>,
    active_tetromino: Tetromino,
}

impl Tetris {
    fn new(width: usize, height: usize) -> Tetris {
        let mut initial_tetromino = Tetromino::pick_any();
        initial_tetromino.x = width as isize / 2;
        initial_tetromino.y = height as isize - 1;
        Tetris {
            game_start: Instant::now(),
            last_drop: Instant::now(),
            active_tetromino: initial_tetromino,
            board: DMatrix::from_element(width, height, None),
        }
    }

    fn width(&self) -> usize {
        self.board.nrows()
    }

    fn height(&self) -> usize {
        self.board.ncols()
    }

    fn dispatch_keypressed_event(&mut self, keycode: u8) {
        let mut move_direction = None;
        match keycode {
            113 => move_direction = Some(MoveDirection::Left),
            114 => move_direction = Some(MoveDirection::Right),
            116 => move_direction = Some(MoveDirection::Down),
            111 => self.try_rotate(),
            keycode => println!("Keycode: {}", keycode),
        };

        if let Some(move_direction) = move_direction {
            self.try_move(move_direction);
        }

    }

    fn try_move(&mut self, move_direction: MoveDirection) {
        let next_tetromino_candidate = self.active_tetromino.moved_by(move_direction);
        if !self.has_collision(&next_tetromino_candidate) {
            self.active_tetromino = next_tetromino_candidate;

        } else if move_direction == MoveDirection::Down {
            self.solidify();
        }
    }

    fn try_rotate(&mut self) {
        let rotated = self.active_tetromino.rotated();
        if !self.has_collision(&rotated) {
            self.active_tetromino = rotated;
        }
    }

    fn solidify(&mut self) {
        for (x, y) in self.active_tetromino.coords() {
            self.board[(x as usize, y as usize)] = Some(BoardCell::new(x, y, self.active_tetromino.color));
        }
        self.active_tetromino = Tetromino::pick_any();
        self.active_tetromino.x = self.width() as isize / 2;
        self.active_tetromino.y = self.height() as isize - 1;
    }

    fn has_collision(&self, test_tetromino: &Tetromino) -> bool {
        for (x, y) in test_tetromino.coords() {
            let in_bounds = x >= 0 && x < self.width() as isize && y >= 0 && y < self.height() as isize;
            if !in_bounds || self.board[(x as usize, y as usize)].is_some() {
                return true;
            }
        }
        false
    }

    fn game_rotation(&self) -> Trans3d {
        let elapsed = self.game_start.elapsed();
        let elapsed_secs = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1.0e-9;
        Trans3d::from_angle_x(Deg(60.0 * elapsed_secs))
    }

    fn render_falling(&self) -> Vec<RenderOptions> {
        let mut render_options = Vec::new();
        for (x, y) in self.active_tetromino.coords() {
            render_options.push(RenderOptions::new(place_at(x, y), Some(self.active_tetromino.color)));
        }
        render_options
    }

    fn render_solid(&self) -> Vec<RenderOptions> {
        let mut render_options = Vec::new();
        for x in 0..self.width() {
            for y in 0..self.height() {
                if let Some(cell) = self.board[(x as usize, y as usize)] {
                    render_options.push(RenderOptions::new(cell.transform, Some(cell.color)));
                }
            }
        }
        render_options
    }

    fn render_border(&self) -> Vec<RenderOptions> {
        let mut render_options = Vec::new();
        for x in -1..self.width() as isize + 1 {
            render_options.push(RenderOptions::new(place_at(x, -1), None));
        }
        for y in 0..self.height() as isize {
            render_options.push(RenderOptions::new(place_at(-1, y), None));
            render_options.push(RenderOptions::new(place_at(self.width() as isize, y), None));
        }
        render_options
    }
}

impl Controller<GameElement> for Tetris {
    fn get_transform(&self) -> Trans4d {
        let left_border = -1.0;
        let right_border = self.width() as f32;
        let bottom_border = -1.0;
        let top_border = (self.height() - 1) as f32;

        Trans4d::from_translation(Coord3d::new(0.0, 0.0, (top_border - bottom_border) * -2.0 - 0.5)) * Trans4d::from(self.game_rotation()) *
        Trans4d::from_translation(Coord3d::new((left_border + right_border) / -2.0, (bottom_border + top_border) / -2.0, 0.0))
    }

    fn get_light_direction(&self) -> Coord3d {
        self.game_rotation().transpose() * Coord3d::new(-1.0, -0.75, -0.5)
    }

    fn prepare_state(&mut self, events: PollEventsIterator) -> bool {
        for event in events {
            match event {
                Event::Closed => return false,
                Event::KeyboardInput(ElementState::Pressed, keycode, _) => self.dispatch_keypressed_event(keycode),
                _ => {}
            }
        }
        if self.last_drop.elapsed().as_secs() >= 1 {
            self.try_move(MoveDirection::Down);
            self.last_drop = Instant::now();
        }
        return true;
    }

    fn get_render_options(&self, object: &GameElement) -> Vec<RenderOptions> {
        match object {
            &GameElement::Falling => self.render_falling(),
            &GameElement::Solid => self.render_solid(),
            &GameElement::Border => self.render_border(),
        }
    }
}

fn place_at(x: isize, y: isize) -> Trans4d {
    Trans4d::from_translation(Coord3d::new(x as f32, y as f32, 0.0)) * Trans4d::from_scale(0.45)
}

enum GameElement {
    Falling,
    Solid,
    Border,
}
