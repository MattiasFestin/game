pub const CHUNK_SIZE: usize = 10;
pub const CHUNK_SIZE_CUBE: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNKS_LOADED: usize = 9;


pub const MIN_FORCE: f64 = 0.01;
pub const MIN_VELOCITY: f64 = 0.01;
pub const MAX_MASS: f64 = 1.0e32;
pub const MAX_SPEED: f64 = PHYSICS_C * 0.1;
// pub const MAX_MASS: f32 = 5.9673e27;

pub const GRAVITY_MIN_MASS: f64 = 12240.4430651;
pub const GRAVITY_MIN_MASS_PROD: f64 = 149828446.430;
pub const GRAVITY_MIN_DISTANCE: f64 = 1.0;
pub const GRAVITY_MAX_DISTANCE: f64 = 150.0e9;

pub const PHYSICS_TICKS: f64 = 0.03333333333;
pub const PHYSICS_GRAVITY: f64 = 6.67430e-11;

pub const PHYSICS_C: f64 = 299792458.0;

pub const GLOBAL_SCALE: f32 = 1.0e11;