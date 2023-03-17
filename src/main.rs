use macroquad::prelude::*;

const WORLD_WIDTH: f32 = 1600.0;
const WORLD_HEIGHT: f32 = 1000.0;
const MOVE_DECAY_FACTOR: f32 = 0.95;
const MOVE_IMPULSE_SIZE: f32 = 0.4;
const ROTATE_DECAY_FACTOR: f32 = 0.85;
const ROTATE_IMPULSE_SIZE: f32 = 0.4;
const PLAYER_RADIUS: f32 = 30.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "WASM".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {

    let mut player_x: f32 = WORLD_WIDTH / 2.0;
    let mut player_y: f32 = WORLD_HEIGHT / 2.0;
    let mut player_a: f32 = 90.0;
    let mut old_player_x = player_x;
    let mut old_player_y = player_y;
    let mut old_player_a = player_a;

    loop {

        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let mut player_dx = (player_x - old_player_x) * MOVE_DECAY_FACTOR;
        let mut player_dy = (player_y - old_player_y) * MOVE_DECAY_FACTOR;
        let mut player_da = (player_a - old_player_a) * ROTATE_DECAY_FACTOR;

        if is_key_down(KeyCode::Left) {
            player_da += ROTATE_IMPULSE_SIZE;
        }

        if is_key_down(KeyCode::Right) {
            player_da -= ROTATE_IMPULSE_SIZE;
        }

        if is_key_down(KeyCode::Up) {
            player_dx += MOVE_IMPULSE_SIZE * player_a.to_radians().cos();
            player_dy += MOVE_IMPULSE_SIZE * player_a.to_radians().sin();
        }

        if is_key_down(KeyCode::Down) {
            player_dx -= MOVE_IMPULSE_SIZE * player_a.to_radians().cos();
            player_dy -= MOVE_IMPULSE_SIZE * player_a.to_radians().sin();
        }

        old_player_x = player_x;
        old_player_y = player_y;
        old_player_a = player_a;
        player_x += player_dx;
        player_y += player_dy;
        player_a += player_da;

        if player_x < PLAYER_RADIUS {
            player_x = PLAYER_RADIUS;
        }

        if player_x > WORLD_WIDTH - PLAYER_RADIUS {
            player_x = WORLD_WIDTH - PLAYER_RADIUS;
        }

        if player_y < PLAYER_RADIUS {
            player_y = PLAYER_RADIUS;
        }

        if player_y > WORLD_HEIGHT - PLAYER_RADIUS {
            player_y = WORLD_HEIGHT - PLAYER_RADIUS;
        }

        clear_background(BLACK);

        let screen_scale;
        let screen_offset_x;
        let screen_offset_y;

        if screen_width() * WORLD_HEIGHT > screen_height() * WORLD_WIDTH {
            screen_scale = screen_height() / WORLD_HEIGHT;
            screen_offset_x = (screen_width() - screen_scale * WORLD_WIDTH) / 2.0;
            screen_offset_y = 0.0;
        }
        
        else {
            screen_scale = screen_width() / WORLD_WIDTH;
            screen_offset_y = (screen_height() - screen_scale * WORLD_HEIGHT) / 2.0;
            screen_offset_x = 0.0;
        }

        let screen_world_width = WORLD_WIDTH * screen_scale;
        let screen_world_height = WORLD_HEIGHT * screen_scale;
        draw_rectangle(screen_offset_x, screen_offset_y, screen_world_width, screen_world_height, LIGHTGRAY);

        let screen_player_x = player_x * screen_scale + screen_offset_x;
        let screen_player_y = screen_height() - (player_y * screen_scale + screen_offset_y);
        let screen_player_radius = PLAYER_RADIUS * screen_scale;
        draw_circle(screen_player_x, screen_player_y, screen_player_radius, RED);

        let screen_player_eye_x = screen_player_x + screen_player_radius * 0.7 * player_a.to_radians().cos();
        let screen_player_eye_y = screen_player_y - screen_player_radius * 0.7 * player_a.to_radians().sin();
        let screen_player_eye_radius = screen_player_radius * 0.2;
        draw_circle(screen_player_eye_x, screen_player_eye_y, screen_player_eye_radius, WHITE);

        next_frame().await;

    }

}
