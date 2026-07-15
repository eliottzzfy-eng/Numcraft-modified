#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![feature(const_index)]
#![feature(const_trait_impl)]

#[macro_use]
mod nadk;

mod camera;
mod constants;
mod entity;
mod game;
mod game_ui;
mod hud;
mod input_manager;
mod inventory;
mod menu;
mod misc;
mod physic;
mod player;
mod renderer;
mod save_manager;
mod settings;
mod timing;
mod world;

use game::Game;


setup_allocator!();

configure_app!(b"Numcraft\0", 9, "../target/assets/icon.nwi", 3437);

fn show_error(message: &[&str]) {
    let background_color = nadk::display::Color565::from_rgb888(253, 81, 81);
    nadk::display::push_rect_uniform(nadk::display::SCREEN_RECT, background_color);

    let mut y = (constants::rendering::SCREEN_HEIGHT - message.len() * 20) / 2;

    for line in message {
        nadk::display::draw_string(
            line,
            nadk::display::ScreenPoint {
                x: ((320 - line.len() * 10) / 2) as u16,
                y: y as u16,
            },
            true,
            nadk::display::Color565::from_rgb888(0, 0, 0),
            background_color,
        );
        y += 20
    }
}

#[unsafe(no_mangle)]
fn main() {
    init_heap!();

    nadk::utils::wait_ok_released();

    #[cfg(feature="upsilon")]
    if nadk::adresses::heap_size() < 100_000{
        show_error(&["Sorry but Numcraft needs", " Upsilon 1.1.2 dev or newer.", "Please update Upsilon from", "getupsilon.web.app/install", "", "Press [Ok] to quit."]);
        nadk::keyboard::wait_until_pressed(nadk::keyboard::Key::Ok);
        return ;
    }

    let mut game = Game::new();

    game.main_loop();
}
