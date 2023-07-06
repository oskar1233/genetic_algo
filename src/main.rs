pub mod genetics;
pub mod graphics;

struct AppLogic {
    genetics: genetics::Genetics,
}

impl graphics::Logic for AppLogic {
    fn update(&mut self) {
        self.genetics.evolve();
    }

    fn get_gen(&mut self) -> (usize, Vec<bool>) {
        match self.genetics.get_last_population() {
            Some((count, population)) => (count, population),
            None => (0, vec![]),
        }
    }
}

impl AppLogic {
    fn start() -> Self {
        Self {
            genetics: genetics::Genetics::start(),
        }
    }
}

fn main() {
    let app_logic = AppLogic::start();
    let mut graphics = graphics::Graphics::start(app_logic);

    graphics.run();
}
