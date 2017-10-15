
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;

use game::{Game, Direction};
use constants::*;
use TimeUpdate;

#[derive(PartialEq, Eq)]
pub enum AppState {
    Start,
    Game,
    GameOver,
    Quit
}

pub fn start(events: &mut Iterator<Item=Event>, time_update: TimeUpdate, game: &mut Game) -> (AppState, bool) {
    for event in events {
        match event {
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                return (AppState::Quit, false);
            },
            Event::KeyDown {..} => {
                game.reset();
                return (AppState::Game, true);
            },
            _ => {}
        }
    }
    let previous = (time_update.absolute - time_update.diff) / START_SCREEN_SPEED;
    let current = time_update.absolute / START_SCREEN_SPEED;
    (AppState::Start, previous != current)
}

pub fn game(events: &mut Iterator<Item=Event>, time_update: TimeUpdate, game: &mut Game) -> (AppState, bool) {
    let mut redraw_needed = false;
    for event in events {
        let mut direction = None;
        match event {
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                game.reset();
                return (AppState::Start, true);
            },
            Event::KeyDown {keycode: Some(Keycode::Up), ..} => { direction = Some(Direction::Up); },
            Event::KeyDown {keycode: Some(Keycode::Down), ..} => { direction = Some(Direction::Down); },
            Event::KeyDown {keycode: Some(Keycode::Right), ..} => { direction = Some(Direction::Right); },
            Event::KeyDown {keycode: Some(Keycode::Left), ..} => { direction = Some(Direction::Left); },
            Event::Window {win_event: WindowEvent::Exposed, ..} => { redraw_needed = true; },
            _ => {}
        }
        if let Some(direction) = direction {
            game.update_direction(direction);
        }
    }
    if !game.alive {
        return (AppState::GameOver, true);
    }
    redraw_needed = game.tick(time_update.diff) || redraw_needed;
    (AppState::Game, redraw_needed)
}

pub fn game_over(events: &mut Iterator<Item=Event>, time_update: TimeUpdate) -> (AppState, bool) {
    for event in events {
        match event {
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                return (AppState::Start, false);
            },
            _ => {}
        }
    }
    if time_update.absolute > GAME_OVER_SCREEN_DURATION {
        return (AppState::Start, false);
    }
    let previous = (time_update.absolute - time_update.diff) / GAME_OVER_SCREEN_SPEED;
    let current = time_update.absolute / GAME_OVER_SCREEN_SPEED;
    (AppState::GameOver, previous != current)
}
