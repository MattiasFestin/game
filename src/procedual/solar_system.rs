use bevy::pbr::AmbientLight;
use bevy::prelude::*;
use bevy::math::{Vec2, Vec3, Vec3A};

use crate::constants::{GLOBAL_SCALE, PHYSICS_GRAVITY};

static SUN_MASS: f64 = 1.989e30;        //Kg
static SUN_RADIUS: f64 = 6.963400e8;   //m
static SUN_LUMINOSITY: f64 = 3.828e26;  //W
static SUN_TEMPERATURE: f64 = 5778.0;   //K

static AU: f64 = 1.496e11;
static EARTH_MASS: f64 = 5.9722e24;
static EARTH_RADIUS: f64 = 6.3781e6;

static MIN_STAR_MASS: f64 = 0.064 * SUN_MASS;
static MAX_STAR_MASS: f64 = 265.0 * SUN_MASS;

static MAX_SOLID_PLANET_MASS: f64 = 50.0 * EARTH_MASS;
static MIN_GAS_PLANET_MASS: f64 = 125.0 * EARTH_MASS;

static σ: f64 = 5.670374419e-8;
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
    let s = d / AU;

    return -0.0038 * s * s * s + 0.1236 * s * s - 1.5275 * s + 6.7796;
}

fn planet_temperature(l: f64, a: f64, d2: f64) -> f64 {
    return (l * (1.0 - a))/(16.0 * std::f64::consts::PI  * σ * d2);
}

fn star_luminosity(m: f64) -> f64 {
    let a: f64;
    let b: f64;
    let m_norm = m / SUN_MASS;

    if m_norm > 0.2 && m_norm < 0.85 {
        let m2 = m_norm * m_norm;
        let m3 = m2 * m_norm;
        let m4 = m3 * m_norm;
        a = -141.7 * m4 + 232.4 * m3 - 129.1 * m2 + 33.29 * m_norm + 0.215;
        b = 1.0;
    } else if m_norm < 2.0 {
        a = 4.0;
        b = 1.0;
    } else if m_norm < 55.0 {
        a = 3.5;
        b = 1.4;
    } else {
        a = 1.0;
        b = 32000.0;
    }
    
    return b * SUN_LUMINOSITY * m_norm.powf(a);
}

fn star_temperature(m: f64, r: f64) -> f64 {
    let l = star_luminosity(m);
    return (l / (4.0 * std::f64::consts::PI * r * r * σ)).powf(0.25);
}

fn luminosity_at(p: Vec3A, sun_p: Vec3A, m: f64, r: f64) -> f64 {
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
    if m < MAX_SOLID_PLANET_MASS {
        return EARTH_RADIUS * m.powf(0.56);
    }
    //  else { // if m > MIN_GAS_PLANET_MASS {
    return EARTH_RADIUS * m.powf(0.02);
    // }
 
    // return 0.0;
}

fn star_radius(m: f64) -> f64 {
    let m_norm = (m / SUN_MASS);
    let e = crate::easing::lerp(
        Vec2::new(0.0, 0.57),
        Vec2::new(1.5, 0.8),
        m_norm as f32
    ).clamp(0.57, 0.8);

    return SUN_RADIUS * m_norm.powf(e as f64);
}

#[derive(Debug, Clone, Copy)]
pub struct Star {
    pub id: u64,
    pub position: Vec3A,
    pub mass: f64,            //Kg
    pub radius: f64,      //m
    pub temperature: f64, //Kelvin
    pub luminosity: f64,  //Watt
    pub color: Color,
    // pub cycle: f64                //% of max luminosity
}

impl Star {
    fn create(x: u64, y: u64, z: u64, seed: u64) -> Self {
        let id = crate::noise::noise_3d(x, y, z, seed);
        let mass = MAX_STAR_MASS * ((crate::noise::noise_3d(x, y, z, seed.wrapping_add(id)) as f64) / u64::MAX as f64) + MIN_STAR_MASS;
        let position = Vec3A::new(x as f32, y as f32, z as f32);
        let radius = star_radius(mass);
        let luminosity = star_luminosity(mass);
        let temperature = star_temperature(mass, radius);
        let color = crate::physics::plancks_law_rgb(temperature);

        Self {
            id,
            position,
            mass: mass,
            radius,
            temperature,
            luminosity,
            color
        }
    }

    fn luminosity_at(&mut self, p: Vec3A) -> f64 {
        return luminosity_at(p, self.position, self.mass, self.radius);
    }

    fn pbr(&self) -> StandardMaterial {
        return StandardMaterial {
            base_color: self.color,
            emissive: self.color,
            double_sided: true,
            reflectance: 0.0,
            roughness: 1.0,
           
            ..Default::default()
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Planet {
    pub id: u64,
    pub gas: bool,
    pub position: Vec3A,
    pub mass: f64,        //Kg
    pub veclocity: Vec3A,
    pub albedo: f64,
    pub radius: f64,      //m
    pub temperature: f64, //Kelvin
    pub density: f64,     //Kg/m3
}

impl Planet {
    fn create(star: &Star, x: u64, y: u64, z: u64, seed: u64) -> Self {
        let id = crate::noise::noise_3d(x, y, z, seed);
        let albedo = crate::noise::noise_3d_f64_normalized(x, y, z, seed);

        let position = Vec3A::new(x as f32, y as f32, z as f32);
        let d = star.position.distance(position);
        let density = planet_density_distribution(d as f64).clamp(0.5,7.0);

        let gas = density < 3.0;

        let mass: f64;

        if gas {
            mass = 350.0 * EARTH_MASS * (crate::noise::noise_3d(x, y, z, seed.wrapping_add(id)) as f64) / (u64::MAX as f64) + MIN_GAS_PLANET_MASS;
        } else {
            mass = MAX_SOLID_PLANET_MASS * (crate::noise::noise_3d(x, y, z, seed.wrapping_add(id)) as f64) / (u64::MAX as f64) + 0.055 * EARTH_MASS;
        }

        
        let radius = planet_radius(mass);
        let temperature = planet_temperature(star.luminosity, albedo, (d*d) as f64);

        let veclocity = (position - star.position).any_orthonormal_vector() * ((PHYSICS_GRAVITY * star.mass) as f32) / d;
        
        //TODO:
        assert!(veclocity.dot(position - star.position) < 0.0001, "velocity should be orhogonal");

        return Self {
            id,
            gas,
            albedo,
            position,
            mass: SUN_MASS,
            veclocity: veclocity,
            radius: radius,
            temperature: temperature,
            density
        };
    }

    fn pbr(&self) -> StandardMaterial {
        return StandardMaterial {
            base_color: Color::GREEN,
            double_sided: true,
            reflectance: self.albedo as f32,
            // roughness: 1.0,
           
            ..Default::default()
        };
    }
}

#[derive(Debug, Clone)]
pub struct StarSystem {
    pub star: Star,
    pub planets: Vec<Planet>
}

impl StarSystem {
    fn create(x: u64, y: u64, z: u64, seed: u64) -> Self {
        let star= Star::create(x, y, z, seed);
        let star_r = star.radius;

        let nbr_planets = crate::noise::noise_3d(x, y, z, seed) % 10; //TODO: Max number of planets constant

        let mut planets = Vec::new();
        let mut d = star_r + 0.05 * AU;
        for i in 0..nbr_planets {
            d += crate::noise::noise_3d_f64_normalized(x, y, z, seed + i + 1337) * AU;
            planets.push(Planet::create(&star, x + (AU * star_r / SUN_RADIUS) as u64, y, z, seed));
        }

        return Self {
            star,
            planets
        }
    }
}

fn render_solar_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let mut system = StarSystem::create(0, 0, 0, 42);
    system.planets[0].position = Vec3A::new(50.0, 0.0, 0.0) * GLOBAL_SCALE;
    // system.planets[0].radius *= 10.0;
    // sun.radius = Some(1.0e2);
    // system.star.position = Vec3A::new(2.0 * system.star.radius as f32, 0.0, -AU as f32) ;

    println!("{:?}", system);
    
    let mut pos = system.star.position.into();
    pos = pos / GLOBAL_SCALE;
    commands
        .spawn()
        .insert(system.star)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: (system.star.radius as f32) / GLOBAL_SCALE, subdivisions: 10 })),
            material:  materials.add(system.star.pbr()),
            global_transform: GlobalTransform::from_translation(pos),
            transform: Transform::from_translation(pos),
            ..Default::default()
        })
        .insert_bundle(LightBundle {
            light: Light {
                color: system.star.color,
                fov: f32::to_radians(360.0),
                intensity: 255.0 * (system.star.luminosity / SUN_LUMINOSITY) as f32,
                range: 1.0,
                depth: 0.0..f32::MAX,
                ..Default::default()
            },
            global_transform: GlobalTransform::from_translation(pos),
            transform: Transform::from_translation(pos),
            ..Default::default()
        })
        .insert(bevy_frustum_culling::aabb::Aabb::default())
        ;

    for planet in system.planets {
        let mut pos = planet.position.into();
        pos = pos / GLOBAL_SCALE;
        commands
            .spawn()
            .insert(planet)
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: (planet.radius as f32) / GLOBAL_SCALE * 1000.0, subdivisions: 4 })),
                material:  materials.add(planet.pbr()),
                global_transform: GlobalTransform::from_translation(pos),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(bevy_frustum_culling::aabb::Aabb::default())
            .insert_bundle(bevy_rapier3d::physics::RigidBodyBundle {
                position: pos.into(),
                velocity: bevy_rapier3d::prelude::RigidBodyVelocity { 
                    linvel: (planet.veclocity / GLOBAL_SCALE).into(),
                    angvel: Vec3A::ZERO.into()
                },
                forces: bevy_rapier3d::prelude::RigidBodyForces { gravity_scale: 0.0, ..Default::default() },
                activation: bevy_rapier3d::prelude::RigidBodyActivation::cannot_sleep(),
                ccd: bevy_rapier3d::prelude::RigidBodyCcd { ccd_enabled: false, ..Default::default() },
                ..Default::default()
            })
        // .insert_bundle(bevy_rapier3d::physics::ColliderBundle {
        //     shape: bevy_rapier3d::prelude::ColliderShape::cuboid(1.0, 1.0, 1.0),
        //     collider_type: bevy_rapier3d::prelude::ColliderType::Sensor,
        //     position: ((planet.position / GLOBAL_SCALE).into(), Quat::from_rotation_x(0.0)).into(),
        //     material: bevy_rapier3d::prelude::ColliderMaterial { friction: 0.7, restitution: 0.3, ..Default::default() },
        //     mass_properties: bevy_rapier3d::prelude::ColliderMassProps::Density(2.0),
        //     ..Default::default()
        // })
        .insert(bevy_rapier3d::physics::RigidBodyPositionSync::Discrete)
        ;
    }
}

// fn create_planet(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {

// }

pub fn create(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    render_solar_system(commands, materials, meshes);
}