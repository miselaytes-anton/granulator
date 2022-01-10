use rand::Rng;

pub struct Scheduler {
    next_onset: usize,
    density: f32,
}

impl Scheduler {
    pub fn new(density: f32) -> Scheduler {
        Scheduler {
            next_onset: 0,
            density,
        }
    }

    /**
     * Advances scheduler and returns a bool telling whether
     * new grain should be activated.
     */
    pub fn advance(&mut self) -> bool {
        if self.next_onset == 0 {
            self.next_onset += self.calculate_next_interonset();
            return true;
        }
        self.next_onset -= 1;

        false
    }

    /**
     * Calculates number of samples after which a new grain
     * should be activated. Calculation is based on density.
     */
    fn calculate_next_interonset(&self) -> usize {
        let mut rng = rand::thread_rng();
        let random: f32 = rng.gen_range(0.1..1.0);
        let interonset = -(random.ln() / self.density * 1000.0).ceil() as usize;
        if interonset == 0 {
            return 1;
        }
        interonset
    }

    pub fn set_density(&mut self, density: f32) {
        self.density = density;
    }
}
