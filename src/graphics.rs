
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use constants::*;
use text::GlyphAtlas;
use game::Game;
use TimeUpdate;
//use TimeUpdate;

fn rect(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect::new(x as i32, y as i32, w, h)
}

pub fn draw_start_screen(canvas: &mut WindowCanvas, atlas: &mut GlyphAtlas, time_update: TimeUpdate) {
    setup_full_viewport(canvas);
    let position = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    // Hacky way to avoid bringing in arrayvec
    let mut chars: [_; 4] = [(0, (Rect::new(0, 0, 0, 0), Rect::new(0, 0, 0, 0))); 4];
    for item in atlas.text_rects_centered("Mato", position, 6).enumerate() {
        chars[item.0] = item;
    }
    for &(i, (src, dst)) in &chars[..] {
        if (time_update.absolute / START_SCREEN_SPEED) % 4 == i as u32 {
        //if (start_counter >> i) & 1 == 1 {
            atlas.set_render_color(TITLE_COLOR_2.0, TITLE_COLOR_2.1, TITLE_COLOR_2.2);
        } else {
            atlas.set_render_color(TITLE_COLOR_1.0, TITLE_COLOR_1.1, TITLE_COLOR_1.2);
        }
        canvas.copy(&atlas.texture(), src, dst).expect("Could not draw glyph");
    }
}

pub fn draw_game(canvas: &mut WindowCanvas, game: &Game, atlas: &mut GlyphAtlas) {
    setup_status_bar_viewport(canvas);
    draw_points(canvas, atlas, game.points);

    setup_game_viewport(canvas);
    draw_arena(canvas);
    draw_apples(canvas, &game.apples);
    draw_worm(canvas, &game.worm);
}

pub fn draw_game_over(canvas: &mut WindowCanvas, game: &Game, atlas: &mut GlyphAtlas, time_update: TimeUpdate) {
    setup_status_bar_viewport(canvas);
    draw_points(canvas, atlas, game.points);

    setup_game_viewport(canvas);
    draw_arena(canvas);
    draw_apples(canvas, &game.apples);
    //draw_worm(canvas, &game.worm);

    let dead_segment = if time_update.absolute > GAME_OVER_SCREEN_DELAY {
        (time_update.absolute - GAME_OVER_SCREEN_DELAY) / GAME_OVER_SCREEN_SPEED
    } else {
        0
    };
    for (i, segment) in game.worm.iter().enumerate() {
        if dead_segment > i as u32 {

            //let r = rect(x, y, size, size);
            //canvas.set_draw_color(WORM_COLOR);
            //canvas.fill_rect(r).expect("Drawing worm");

            canvas.set_draw_color(Color { r: 255, g: 255, b: 255, a: 255 });
            let size = BLOCK_SIZE / 2;
            let offset = BLOCK_SIZE / 4;
            let x = segment.0 * BLOCK_SIZE + offset;
            let y = segment.1 * BLOCK_SIZE + offset;
            let r = rect(x, y, size, size);
            canvas.fill_rect(r).expect("Drawing worm");
        } else {
            canvas.set_draw_color(WORM_COLOR);
            let x = segment.0 * BLOCK_SIZE + 1;
            let y = segment.1 * BLOCK_SIZE + 1;
            let size = BLOCK_SIZE - 2;
            let r = rect(x, y, size, size);
            canvas.fill_rect(r).expect("Drawing worm");
        };
    }

    setup_full_viewport(canvas);
    let bg = (BACKGROUND_COLOR.r, BACKGROUND_COLOR.g, BACKGROUND_COLOR.b);
    let fg = TITLE_COLOR_2;
    let game_over_scale = 3;
    // Draw the text multiple times with black to give borders to it
    let text_repetitions = [
        (-1, -1, bg),
        (0, -1, bg),
        (1, -1, bg),

        (-1, 0, bg),
        (1, 0, bg),

        (-1, 1, bg),
        (0, 1, bg),
        (1, 1, bg),

        (0, 0, fg)
    ];
    for &(x, y, color) in &text_repetitions {
        let x = x * game_over_scale;
        let y = y * game_over_scale;
        let position = (WINDOW_WIDTH as i32 / 2 + x, WINDOW_HEIGHT as i32 / 2 + y);
        atlas.set_render_color(color.0, color.1, color.2);
        for (src, dst) in atlas.text_rects_centered("GAME OVER", position, game_over_scale as u32) {
            canvas.copy(&atlas.texture(), src, dst).expect("Could not draw glyph");
        }
    }
    //let position = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    //for (src, dst) in atlas.text_rects_centered("GAME OVER", position, 3) {
    //    canvas.copy(&atlas.texture(), src, dst).expect("Could not draw glyph");
    //}
}

fn setup_full_viewport(canvas: &mut WindowCanvas) {
    canvas.set_viewport(rect(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT));
}

fn setup_game_viewport(canvas: &mut WindowCanvas) {
    canvas.set_viewport(rect(0, STATUS_BAR_HEIGHT, ARENA_WIDTH_PX, ARENA_HEIGHT_PX));
}

fn setup_status_bar_viewport(canvas: &mut WindowCanvas) {
    canvas.set_viewport(rect(0, 0, WINDOW_WIDTH, STATUS_BAR_HEIGHT));
}

fn draw_points(canvas: &mut WindowCanvas, atlas: &mut GlyphAtlas, points: u32) {
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut text_bytes = [' ' as u8; 10];
    let text_bytes_len = text_bytes.len();
    let mut remaining_points = points;
    for i in 0..text_bytes.len() {
        let digit_int = remaining_points % 10;
        let digit = digits[digit_int as usize];
        text_bytes[text_bytes_len - i - 1] = digit as u8;
        remaining_points = remaining_points / 10;
    }
    //let position = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    let is_nonzero_digit = |c: &u8| *c != ' ' as u8 && *c != '0' as u8;
    let start = text_bytes.iter().position(is_nonzero_digit).unwrap_or(text_bytes.len() - 1);
    atlas.set_render_color(POINTS_COLOR.0, POINTS_COLOR.1, POINTS_COLOR.2);
    for (src, dst) in atlas.text_rects_right_aligned(&text_bytes[start..], (WINDOW_WIDTH as i32, 0), 2) {
        canvas.copy(&atlas.texture(), src, dst).expect("Could not draw glyph");
    }
}

fn draw_arena(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(WALL_COLOR);
    canvas.fill_rects(&[
        rect(0, 0, ARENA_WIDTH_PX, BLOCK_SIZE),
        rect(0, ARENA_HEIGHT_PX - BLOCK_SIZE, ARENA_WIDTH_PX, BLOCK_SIZE),
        rect(0, 0, BLOCK_SIZE, ARENA_HEIGHT_PX),
        rect(ARENA_WIDTH_PX - BLOCK_SIZE, 0, BLOCK_SIZE, ARENA_HEIGHT_PX),
    ]).expect("Drawing walls");
}

fn draw_apples(canvas: &mut WindowCanvas, apples: &[(u32, u32)]) {
    canvas.set_draw_color(APPLE_COLOR);
    for pos in apples.iter() {
        canvas.fill_rect(rect(pos.0 * BLOCK_SIZE + 1, pos.1 * BLOCK_SIZE + 1, BLOCK_SIZE - 2, BLOCK_SIZE - 2)).expect("Drawing apple");
    }
}

fn draw_worm(canvas: &mut WindowCanvas, worm: &Vec<(u32, u32)>) {
    canvas.set_draw_color(WORM_COLOR);
    for segment in worm.iter() {
        canvas.fill_rect(rect(segment.0 * BLOCK_SIZE + 1, segment.1 * BLOCK_SIZE + 1, BLOCK_SIZE - 2, BLOCK_SIZE - 2)).expect("Drawing worm");
    }
}
