use screen::Screen;
use std::thread::sleep;
use std::time::Duration;
use piston_window;
use loader;
use MyWindow;

pub struct BirthScreen {
    progress: f32,

    loader: loader::Loader<()>,
}

fn load(tick: loader::Ticker) -> () {
    for _ in 0..500 {
        tick();
        sleep(Duration::from_millis(1));
    }
}

impl BirthScreen {
    pub fn new() -> Self {
        BirthScreen {
            progress: 0f32,
            loader: loader::Loader::new(load, loader::Linearizer::simple(500)),
        }
    }

    pub fn finish(self) -> () {
        self.loader.expect()
    }
}

impl Screen for BirthScreen {
    fn render(&mut self, args: &piston_window::RenderArgs, e: MyWindow) {
        e.draw_2d(|c, gl| {
            let gray = self.progress * self.progress;
            piston_window::clear([gray, gray, gray, 1f32], gl);
        });
    }

    fn update(&mut self, args: &piston_window::UpdateArgs) {
        match self.loader.progress() {
            loader::LoadProgress::Loading(progress) =>
                self.progress = progress,
                loader::LoadProgress::Ready => (),
        }
    }

    fn is_running(&self) -> bool {
        !self.loader.is_ready()
    }
}
