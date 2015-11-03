use piston_window;

use MyWindow;

pub trait Screen {
    fn render(&mut self, args: &piston_window::RenderArgs, e: MyWindow);
    fn update(&mut self, args: &piston_window::UpdateArgs) {}
    fn input(&mut self, args: &piston_window::Input) {}
    fn is_running(&self) -> bool { true }

    fn event_loop(&mut self, window: MyWindow) {
        for e in window {
            if !self.is_running() {
                break;
            }
            match e.event {
                Some(piston_window::Event::Render(r)) => self.render(&r, e),
                Some(piston_window::Event::Update(u)) => self.update(&u),
                Some(piston_window::Event::Input(i)) => self.input(&i),
                Some(piston_window::Event::Idle(_)) => (),
                Some(piston_window::Event::AfterRender(_)) => (),
                None => (),
            };
        }
    }
}
