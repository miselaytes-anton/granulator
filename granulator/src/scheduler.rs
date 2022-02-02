use libm::{ceilf, logf};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Scheduler {
    next_onset: usize,
    density: f32,
    small_rng: SmallRng,
}

impl Scheduler {
    pub fn new(density: f32) -> Scheduler {
        Scheduler {
            next_onset: 0,
            density,
            small_rng: SmallRng::seed_from_u64(10),
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
    fn calculate_next_interonset(&mut self) -> usize {
        let random: f32 = self.small_rng.gen_range(0.1..1.0);

        let interonset = -ceilf(logf(random) / self.density * 1000.0f32) as usize;
        if interonset == 0 {
            return 1;
        }
        interonset
    }

    pub fn set_density(&mut self, density: f32) {
        self.density = density;
    }
}
