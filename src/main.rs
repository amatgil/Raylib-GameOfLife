use ray_game_of_life::{Cell, Coord, Universe};
use macroquad::{prelude::*, text};



#[macroquad::main(window_conf)]
async fn main() {
    // Variables
    let background_color         = Color::from_rgba(24, 25, 38, 255);
    let grid_thickness           = 2.5;
    let grid_color               = Color::from_rgba(138, 173, 244, 255);
    let grid_spacing             = 40;
    let alive_color              = Color::from_rgba(145, 215, 227, 255); // Lavander
    let dead_color               = Color::from_rgba(0, 0, 0, 0);         // Transparent
    let text_color               = Color::from_rgba(198, 160, 246, 200);
    let mut time_between_ticks   = 0.3;                                  // In seconds;
    let time_between_ticks_delta = 0.01;                                 // In seconds

    let uni_width = || screen_width() as usize / grid_spacing;
    let uni_height = || screen_height() as usize / grid_spacing;
    let bare_universe = || Universe::new(uni_width(), uni_height());

    // Simulation
    let mut universe = bare_universe();
    let mut paused = true;
    let mut frames_since_last_tick = 0;

    // Main loop
    loop {
        clear_background(background_color);
        universe.set_dimensions(Coord::new(uni_width(), uni_height()));

        let time_since_last_tick = frames_since_last_tick as f32 / get_fps() as f32;

        if !paused && (time_between_ticks < time_since_last_tick) {
            frames_since_last_tick = 0;
            universe.tick();
        } else {
            frames_since_last_tick += 1;
        }

        if is_key_pressed(KeyCode::Space) { paused = !paused; }
        if is_key_down(KeyCode::R)        { universe = bare_universe(); }
        if is_key_down(KeyCode::D)        { time_between_ticks += time_between_ticks_delta; }
        if is_key_down(KeyCode::U)        {
            time_between_ticks = (time_between_ticks - time_between_ticks_delta).max(0.0);
        }
        if is_mouse_button_down(MouseButton::Left) {
            let (globl_x, globl_y) = mouse_position();
            universe.set_pixel(Coord::new(
                (globl_x as usize / grid_spacing).min(uni_width() - 1),
                (globl_y as usize / grid_spacing).min(uni_height() - 1)),
                               Cell::Alive);
        }

        draw_universe(&universe, grid_spacing, alive_color, dead_color);
        draw_grid(grid_thickness, grid_color, grid_spacing);
        draw_controls(text_color, time_between_ticks, paused);

        next_frame().await
    }
}

fn draw_controls(text_color: Color, time_between_ticks: f32, paused: bool) {
    let tps = (time_between_ticks + 1.0) / (1.0/get_fps() as f32 + time_between_ticks);
    let is_p = if paused { "Y" } else { "N" };

    draw_rectangle(0.0, 0.0,
                    320.0, 40.0*6.0,
                    Color::from_rgba(0, 0, 0, 200));
    draw_text("U: Increase Speed",                     10.0, 30.0 + 0.0*40.0, 40.0, text_color);
    draw_text("D: Decrease Speed",                     10.0, 30.0 + 1.0*40.0, 40.0, text_color);
    draw_text("R: Reset",                              10.0, 30.0 + 2.0*40.0, 40.0, text_color);
    draw_text(&format!("Space: Pause ({is_p})"),       10.0, 30.0 + 3.0*40.0, 40.0, text_color);
    draw_text(&format!("Speed: {tps:.2} tps",),    10.0, 30.0 + 5.0*40.0, 40.0, text_color);
}

fn draw_universe(universe: &Universe, grid_spacing: usize, alive_color: Color, dead_color: Color) {
    for (i, _) in universe.cells.iter().enumerate() {
    let c@Coord { row, col } = universe.idx_to_coords(i);
        let x = row * grid_spacing;
        let y = col * grid_spacing;

        let cell_color = 
            if universe.is_alive(c) { alive_color }
        else { dead_color };

        draw_rectangle(x as f32, y as f32, grid_spacing as f32, grid_spacing as f32, cell_color);
    }
}

fn draw_grid(grid_thickness: f32, grid_color: Color, grid_spacing: usize) {
    for y in (0..screen_height() as usize).step_by(grid_spacing) {
        draw_line(0.0, y as f32,
                  screen_width(), y as f32,
                  grid_thickness, grid_color);
    }
    for x in (0..screen_width() as usize).step_by(grid_spacing) {
        draw_line(x as f32, 0.0,
                  x as f32, screen_height(),
                  grid_thickness, grid_color);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1080,
        window_height: 720,
        ..Default::default()
    }
}
