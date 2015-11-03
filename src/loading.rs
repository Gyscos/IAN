use std::thread::sleep;
use std::time::Duration;
use screen::Screen;
use piston_window;
use piston_window::Transformed;
use loader;
use MyWindow;

pub struct LoadingScreen {
    progress: f32,
    loader: loader::Loader<()>,
}

fn load(tick: loader::Ticker) -> () {
    for _ in 0..1000 {
        tick();
        sleep(Duration::from_millis(3));
    }
}

impl LoadingScreen {
    pub fn new() -> Self {
        LoadingScreen {
            progress: 0f32,
            loader: loader::Loader::new(load, loader::Linearizer::simple(1000)),
        }
    }

    pub fn finish(self) -> () {
        self.loader.expect()
    }
}

impl Screen for LoadingScreen {
    fn render(&mut self, args: &piston_window::RenderArgs, e: MyWindow) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let grey = 1f32 - self.progress;
        let grey_color:   [f32; 4] = [grey, grey, grey, 1.0];

        let square = piston_window::rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.progress as f64 * 12f64;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        e.draw_2d(|c, gl| {
            // Clear the screen.
            piston_window::clear(WHITE, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            piston_window::rectangle(grey_color, square, transform, gl);
        });
    }

    fn update(&mut self, args: &piston_window::UpdateArgs) {
        match self.loader.progress() {
            loader::LoadProgress::Loading(progress) =>
                self.progress = progress,
                loader::LoadProgress::Ready => (),
        }
    }

    fn input(&mut self, i: &piston_window::Input) {
        println!("{:?}", i);
    }

    fn is_running(&self) -> bool {
        !self.loader.is_ready()
    }
}
