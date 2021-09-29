use rand::Rng;
pub struct GameState {
    pub seed: u64
}

impl Default for GameState {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self { seed: rng.gen() }
    }
}