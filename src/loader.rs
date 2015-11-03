use std::thread;
use std::sync::RwLock;
use std::sync::Arc;
use time;
use std::sync::atomic::{AtomicUsize,Ordering};
use eventual::{Future, Async};

pub type Ticker<'a> = Box<Fn() + 'a>;

/// Describes the current state of a resource loading.
/// Can be either still loading with a linear progress in [0,1[, or ready.
pub enum LoadProgress {
    Loading(f32),
    Ready,
}

// Small tool to fix a non-linear loader
// It's basically a piecewise linear function that maps
// the non-linear input to a hopefully linear output.
// TODO: derive rustc_decodable & cie
pub struct Linearizer {
    steps: Vec<(usize,f32)>,
}

// Simple tool to interpolate between two points.
fn interpolate((t_a,p_a): (usize,f32), (t_b,p_b): (usize,f32), tick: usize) -> f32 {
    let x = (tick - t_a) as f32 / (t_b - t_a) as f32;
    p_a + x * (p_b - p_a)
}

impl Linearizer {
    fn linearize(&self, ticks: usize) -> f32 {
        match self.steps.binary_search_by(|t| t.0.cmp(&ticks)) {
            Ok(i) => self.steps[i].1,
            Err(i) if i == self.steps.len() => self.steps.last().unwrap().1,
            Err(i) if i == 0 => panic!("is tick negative??"),
            Err(i) => interpolate(self.steps[i-1], self.steps[i], ticks),
        }
    }

    /// Actually run the loader once to learn its progress profile.
    /// TODO: average multiple runs.
    pub fn learn<T, L: FnOnce(Ticker) -> T>(load: L) -> Self {
        let ticks = RwLock::new(Vec::new());

        let start = time::precise_time_ns();
        ticks.write().unwrap().push(start);

        load(Box::new(|| {
            let now = time::precise_time_ns();
            ticks.write().unwrap().push(now);
        }));

        let end = time::precise_time_ns();
        ticks.write().unwrap().push(end);

        let steps = ticks.read().unwrap().iter()
            .map(|t| (t - start) as f32 / (end - start) as f32)
            .enumerate().collect();

        Linearizer {
            steps: steps,
        }
    }

    pub fn simple(max: usize) -> Self{
        Linearizer {
            steps: vec![(0,0f32), (max, 1f32)],
        }
    }
}

/// Run a loading function and provides progress report.
pub struct Loader<T: 'static + Send> {
    ticks: Arc<AtomicUsize>,
    result: Future<T,()>,

    linearizer: Linearizer,
}

impl <T: Send> Loader<T> {
    pub fn new<L: 'static + Send + FnOnce(Ticker) -> T>(load: L, linearizer: Linearizer) -> Self {
        let tick = Arc::new(AtomicUsize::new(0));

        let tock = tick.clone();
        let result = Future::spawn(move || {
            load(Box::new(move || { tock.fetch_add(1, Ordering::Relaxed); }))
        });

        Loader {
            ticks: tick,
            result: result,
            linearizer: linearizer
        }
    }

    pub fn is_ready(&self) -> bool {
        self.result.is_ready()
    }

    pub fn progress(&self) -> LoadProgress {
        if self.is_ready() {
            LoadProgress::Ready
        } else {
            let ticks = self.ticks.load(Ordering::Relaxed);
            LoadProgress::Loading(self.linearizer.linearize(ticks))
        }
    }

    pub fn expect(self) -> T {
        self.result.expect().unwrap()
    }
}
