
extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::event::WindowEvent;

use game::Direction;
use app::AppState;

use constants::*;

mod text;
mod game;
mod app;
mod graphics;

mod constants {
    use sdl2::pixels::Color;

    pub const ARENA_WIDTH_BLOCKS: u32 = 20;
    pub const ARENA_HEIGHT_BLOCKS: u32 = 15;

    pub const TITLE_COLOR_1: (u8, u8, u8) = (0, 100, 100);
    pub const TITLE_COLOR_2: (u8, u8, u8) = (0, 200, 200);
    pub const POINTS_COLOR: (u8, u8, u8) = TITLE_COLOR_2;

    pub const BACKGROUND_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WALL_COLOR: Color = Color { r: 0, g: 0, b: 200, a: 255 };
    pub const APPLE_COLOR: Color = Color { r: 200, g: 0, b: 0, a: 255 };
    pub const WORM_COLOR: Color = Color { r: 0, g: 200, b: 0, a: 255 };

    pub const BLOCK_SIZE: u32 = 16;
    pub const STATUS_BAR_HEIGHT: u32 = BLOCK_SIZE * 2;

    pub const ARENA_WIDTH_PX: u32 = ARENA_WIDTH_BLOCKS * BLOCK_SIZE;
    pub const ARENA_HEIGHT_PX: u32 = ARENA_HEIGHT_BLOCKS * BLOCK_SIZE;

    pub const WINDOW_WIDTH: u32 = ARENA_WIDTH_PX;
    pub const WINDOW_HEIGHT: u32 = ARENA_HEIGHT_PX + STATUS_BAR_HEIGHT;

    pub const INITIAL_STEP_DURATION: u32 = 500;
    pub const MIN_STEP_DURATION: u32 = 100;
    pub const STEP_DURATION_DECREMENT: u32 = 10;

    pub const START_SCREEN_SPEED: u32 = 250;
    pub const GAME_OVER_SCREEN_DELAY: u32 = 1000;
    pub const GAME_OVER_SCREEN_SPEED: u32 = 100;
    pub const GAME_OVER_SCREEN_DURATION: u32 = 15000;
}

#[derive(Clone, Copy)]
pub struct TimeUpdate {
    absolute: u32,
    diff: u32
}

pub struct EventIterator<'a> {
    internal_events: &'a mut sdl2::event::EventPollIterator<'a>,
    redraw_needed: bool,
    quit_requested: bool
}

impl<'a> EventIterator<'a> {
    fn new(internal_events: &'a mut sdl2::event::EventPollIterator<'a>) -> EventIterator<'a> {
        EventIterator {
            internal_events: internal_events,
            redraw_needed: false,
            quit_requested: false,
        }
    }
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = self.internal_events.next();
            match event {
                Some(Event::Quit {..}) => {
                    self.quit_requested = true;
                },
                Some(Event::Window {win_event: WindowEvent::Exposed, ..}) => {
                    self.redraw_needed = true;
                },
                _ => {
                    return event;
                }
            }
        }
    }
}

fn main() {
    let ctx = sdl2::init().expect("Initializing SDL 2");
    let video_ctx = ctx.video().expect("Getting video subsystem");

    let window  = video_ctx
        .window("Mato", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Creating window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Creating canvas");

    let texture_creator = canvas.texture_creator();
    let mut atlas = text::GlyphAtlas::new(&texture_creator);

    let mut timer = ctx.timer().expect("Getting timer subsystem");
    let mut game = game::Game::new(ARENA_WIDTH_BLOCKS, ARENA_HEIGHT_BLOCKS);

    let mut redraw_needed = true;

    let mut events = ctx.event_pump().expect("Getting event pump");
    let mut now = timer.ticks();
    let mut state_start = now;

    let mut state = AppState::Start;

    'main: loop {
        let old_now = now;
        now = timer.ticks();
        let time_diff = now - old_now;
        let time_update = TimeUpdate { absolute: now - state_start, diff: time_diff };

        let mut poll_iter = events.poll_iter();
        let mut event_iterator = EventIterator::new(&mut poll_iter);

        let (new_state, redraw_requested) = match state {
            AppState::Start => app::start(&mut event_iterator, time_update, &mut game),
            AppState::Game => app::game(&mut event_iterator, time_update, &mut game),
            AppState::GameOver => app::game_over(&mut event_iterator, time_update),
            AppState::Quit => break
        };
        if event_iterator.quit_requested {
            break;
        }

        let state_changed = state != new_state;
        state = new_state;
        if state_changed {
            // Run the state updater at least once before calling the render fn
            redraw_needed = true;
            state_start = now;
            continue;
        }
        redraw_needed = redraw_requested || redraw_needed || event_iterator.redraw_needed;

        if redraw_needed {
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.clear();

            match state {
                AppState::Start => graphics::draw_start_screen(&mut canvas, &mut atlas, time_update),
                AppState::Game => graphics::draw_game(&mut canvas, &game, &mut atlas),
                AppState::GameOver => graphics::draw_game_over(&mut canvas, &game, &mut atlas, time_update),
                AppState::Quit => unreachable!()
            }

            redraw_needed = false;

            canvas.present();
        }
    }
}
