use bevy::prelude::*;
use bevy::math::{Vec2, Vec3};

static SUN_MASS: f64 = 1.989e30;        //Kg
static SUN_RADIUS: f64 = 696340000.0;   //m
static SUN_LUMINOSITY: f64 = 3.828e26;  //W
static SUN_TEMPERATURE: f64 = 5778.0;   //K

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
    return SUN_LUMINOSITY * (m / SUN_MASS) *  m.powf(3.5);
}

fn star_temperature(m: f64) -> f64 {
    return SUN_TEMPERATURE * (m / SUN_MASS) *  m.powf(3.5);
}

fn luminosity_at(p: Vec3, sun_p: Vec3, m: f64) -> f64 {
    let l = star_luminosity(m);
    let d = p.distance(sun_p) as f64;
    return l / (4.0 * std::f64::consts::PI * d * d);
}

/*
 * Takes star mass and scales acording to luminosity
 */
 fn scale_to_luminosity(m: f64) -> f64 {
    return AU * star_luminosity(m);
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

struct Star {
    pub position: Vec3,
    pub mass: f64,            //Kg
    radius: Option<f64>,      //m
    temperature: Option<f64>, //Kelvin
    luminosity: Option<f64>,  //Watt
    color: Option<Color>,
    // pub cycle: f64                //% of max luminosity
}

impl Star {
    fn radius(&mut self) -> f64 {
        if self.radius.is_none() {
            self.radius = Some(star_radius(self.mass));
        }

        return self.radius.unwrap();
    }

    fn luminosity(&mut self) -> f64 {
        if self.luminosity.is_none() {
            self.luminosity = Some(star_luminosity(self.mass));
        }

        return self.luminosity.unwrap();
    }

    fn luminosity_at(&mut self, p: Vec3) -> f64 {
        if self.luminosity.is_none() {
            self.luminosity = Some(star_luminosity(self.mass));
        }

        return luminosity_at(p, self.position, self.mass);
    }

    fn temperature(&mut self) -> f64 {
        if self.temperature.is_none() {
            self.temperature = Some(star_temperature(self.mass));
        }

        return self.temperature.unwrap();
    }

    fn color(&mut self) -> Color {
        if self.color.is_none() {
            self.color = Some(crate::physics::plancks_law_rgb(self.temperature()));
        }

        return self.color.unwrap();
    }

    fn pbr(&mut self) -> StandardMaterial {
        return StandardMaterial {
            base_color: self.color(),
            emissive: self.color(),
            double_sided: false,
           
            ..Default::default()
        };
    }
}

impl Default for Star {
    fn default() -> Self {
        Self { 
            position: Vec3::ZERO,
            mass: SUN_MASS,
            radius: Some(SUN_RADIUS),
            temperature: Some(SUN_TEMPERATURE),
            luminosity: Some(SUN_LUMINOSITY),
            color: Some(crate::physics::plancks_law_rgb(SUN_TEMPERATURE))
        }
    }
}




pub fn create(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // let k = 10000.0;
    let mut sun = Star {
        // radius: Some(1.0),
        // position: Vec3::new(10.0, 0.0, 0.0),
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(PbrBundle {
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: sun.radius() as f32, subdivisions: 10 })),
            material:  materials.add(sun.pbr()),
            // m.value().clone(),
            global_transform: GlobalTransform::from_translation(sun.position),
            ..Default::default()
        })
        .insert_bundle(LightBundle {
            light: Light {
                color: sun.color(),
                fov: f32::to_radians(360.0),
                intensity: sun.luminosity() as f32,
                range: f32::MAX,
                ..Default::default()
            },
            transform: Transform::from_translation(sun.position),
            ..Default::default()
        })
        .insert(GlobalTransform::from_translation(sun.position));
}