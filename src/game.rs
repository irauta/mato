
use rand::{ThreadRng, thread_rng};
use rand::distributions::{IndependentSample, Range};

use constants::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    rng: ThreadRng,
    frame_time: u32,
    step_duration: u32,
    pub arena_width: u32,
    pub arena_height: u32,
    pub worm: Vec<(u32, u32)>,
    pub direction: Direction,
    pub new_direction: Option<Direction>,
    pub grow: bool,
    pub apples: Vec<(u32, u32)>,
    pub points: u32,
    pub alive: bool,
}

impl Game {
    pub fn new(arena_width: u32, arena_height: u32) -> Game {
        let worm = {
            let x = arena_width / 2;
            let y = arena_height / 2;
            vec![(x, y)]
        };
        //let worm = (0..8).rev().map(|i| (arena_width / 2 + i, arena_height / 2)).collect();
        let mut game = Game {
            frame_time: 0,
            step_duration: INITIAL_STEP_DURATION,
            arena_width,
            arena_height,
            rng: thread_rng(),
            worm: worm,
            direction: Direction::Right,
            new_direction: None,
            grow: false,
            apples: Vec::new(),
            points: 0,
            alive: true,
        };
        for _ in 0..1 {
            game.add_apple();
        }
        game
    }

    pub fn reset(&mut self) {
        *self = Game::new(self.arena_width, self.arena_height);
    }

    pub fn update_direction(&mut self, new_direction: Direction) {
        if self.new_direction.is_none() && new_direction != self.direction {
            self.new_direction = Some(new_direction);
        }
    }

    pub fn tick(&mut self, time_diff: u32) -> bool {
        self.frame_time += time_diff;
        if self.frame_time < self.step_duration {
            return false;
        }
        self.frame_time -= self.step_duration;
        self.update_worm();
        let current_apple = self.has_apple(self.worm[0]);
        if let Some(i) = current_apple {
            self.apples.remove(i);
            self.add_apple();
            self.grow = true;
            self.points += 5000 / self.step_duration;
            self.step_duration = ::std::cmp::max(self.step_duration - STEP_DURATION_DECREMENT, MIN_STEP_DURATION);
        }
        true
    }

    fn update_worm(&mut self) {
        if !self.alive {
            return;
        }
        if let Some(direction) = self.new_direction {
            self.direction = direction;
            self.new_direction = None;
        }
        use Direction::*;
        if self.worm.len() == 0 {
            panic!("Empty worm!");
        }
        let head = self.worm[0];
        let head = match self.direction {
            Left => (head.0 - 1, head.1),
            Right => (head.0 + 1, head.1),
            Up => (head.0, head.1 - 1),
            Down => (head.0, head.1 + 1),
        };
        if self.head_hits_something(head) {
            self.alive = false;
            return;
        }
        if self.grow {
            self.worm.insert(0, head);
            self.grow = false;
        } else {
            for i in (1..self.worm.len()).rev() {
                self.worm[i] = self.worm[i-1];
            }
            self.worm[0] = head;
        }
    }

    fn head_hits_something(&self, head: (u32, u32)) -> bool {
        head.0 == 0 || head.0 == self.arena_width - 1 ||
        head.1 == 0 || head.1 == self.arena_height - 1 ||
        self.worm[1..].contains(&head)
    }

    fn has_apple(&self, head: (u32, u32)) -> Option<usize> {
        self.apples.iter().position(|apple| head == *apple)
    }

    pub fn add_apple(&mut self) {
        let x_range = Range::new(1, self.arena_width - 1);
        let y_range = Range::new(1, self.arena_height - 1);
        loop {
            let x = x_range.ind_sample(&mut self.rng);
            let y = y_range.ind_sample(&mut self.rng);
            let pos = (x, y);
            if !self.apples.contains(&pos) && !self.worm.contains(&pos) {
                self.apples.push(pos);
                break;
            }
        }
    }
}
