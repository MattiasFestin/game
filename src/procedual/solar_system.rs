use bevy::math::Vec2;

static SUN_MASS: f64 = 1.989e30;
static SUN_RADIUS: f64 = 696340000.0;
static AU: f64 = 1.496e8;
static EARTH_MASS: f64 = 5.9722e24;

static MIN_STAR_MASS: f64 = 0.064 * SUN_MASS;
static MAX_STAR_MASS: f64 = 265.0 * SUN_MASS;

static MAX_SOLID_PLANET_MASS: f64 = 125.0 * EARTH_MASS;
static MIN_GAS_PLANET_MASS: f64 = 125.0 * EARTH_MASS;

// static ASTOID_MIN_DENSITY: f64 = 1.38; // g/cm3
// static ASTOID_MEAN_DENSITY: f64 = 2.0; // g/cm3
// static ASTOID_MAX_DENSITY: f64 = 5.32; // g/cm3

fn astroid_radious(x: f64) -> f64 {
    let a = 14683576.43;
    let b = -1300180.091;

    let n = x.clamp(0.0, 1.0e6) * 1.0e6;
    
    return (0.5*(n-a)/b).exp();
}

fn astroid_density(x: u64, y: u64, z: u64, seed: u64) -> f64 {
    let x = (crate::noise::noise_3d(x, y, z, seed) as f64) / (f64::MAX as f64);
    if x <= 0.5 {
        return 1.38e6 + x * 1.24e6;
    }

    return -1.32e6 + x * 6.64e6;
}

fn mass_from_volume_density(r: f64, d: f64) -> f64  {
    let v = 1.33333333333 * std::f64::consts::PI * r * r * r;

    return d*v;
} 

fn planet_density_distribution(d: f64) -> f64 {
    let s = d * d;

    return -0.0038 * d * s + 0.1236 * s - 1.5275 * d + 6.7796;
}

fn star_luminosity(m: f64) -> f64 {
    m.powf(3.5)
}

/*
 * Takes star mass and scales acording to luminosity
 */
fn scale_to_luminosity(m: f64) -> f64 {
    AU * star_luminosity(m)
}

/*
 * Takes star mass and scales acording to luminosity
 */
 fn scale_luminosity(m: f64) -> f64 {
    AU * star_luminosity(m)
}

fn habital_zone(m: f64) -> (f64, f64) {
    //TODO: Does not take account for atmosspherical pressure
    let au = scale_luminosity(m);
    (0.7 * au, 1.5 * au)
}

fn planet_radius(m: f64) -> f64 {
    //TODO: Planet masses have to have a gap between MIN_GAS_PLANET_MASS
    //TODO: Maybe use density of planet
    if m < MAX_SOLID_PLANET_MASS {
        return m.powf(0.56);
    } else if m > MIN_GAS_PLANET_MASS {
        return m.powf(0.02);
    }
 
    return 0.0;
}

fn star_radius(m: f64) -> f64 {
    let e1 = crate::easing::lerp(
        Vec2::new(MIN_STAR_MASS as f32, 0.57),
        Vec2::new(SUN_MASS as f32, 1.0),
        m as f32
    );
    let e2 = crate::easing::lerp(
        Vec2::new( SUN_MASS as f32, 1.0),
        Vec2::new(MAX_STAR_MASS as f32, 0.8),
        m as f32
    );

    let e = crate::easing::lerp(
        Vec2::new( MIN_STAR_MASS as f32, e1),
        Vec2::new(MAX_STAR_MASS as f32, e2),
        m as f32
    );
    
    m.powf(e as f64) as f64
}

