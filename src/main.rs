extern crate piston_window;
extern crate sdl2_window;
extern crate eventual;
extern crate xdg_basedir;
extern crate time;
extern crate toml;
extern crate flate2;

mod state;
mod loader;
mod screen;
mod birth;
mod loading;

use screen::Screen;
use piston_window::*;
use sdl2_window::Sdl2Window;

type MyWindow = PistonWindow<(),Sdl2Window>;


fn main() {
    // Read CLI parameters, and send it to the app.

    // CLI parameters are pretty simple for now.

    // Now read settings file
    // (Key bindings & stuff)

    let window: MyWindow = WindowSettings::new(
            "IAN",
            [640, 480]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut birth = birth::BirthScreen::new();
    birth.event_loop(window.clone());
    let pre_assets = birth.finish();

    println!("INBETWEEM");

    let mut loading = loading::LoadingScreen::new();
    loading.event_loop(window.clone());
    let assets = loading.finish();

}

