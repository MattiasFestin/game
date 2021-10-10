use std::ops::Deref;

use bevy::math::DQuat;
use bevy::{math::DVec3, prelude::*};
use bevy::render::renderer::RenderResources;
use bevy::reflect::*;
use bevy::tasks::ComputeTaskPool;
use bevy_rapier3d::physics::ColliderBundle;
use bevy_rapier3d::prelude::*;
use crate::utils::reflection::Reflectable;

pub struct Identity {
    pub id: Uuid
}

pub struct Dirty {
    pub is_dirty: bool
}

pub struct Mass {
    pub mass: f64
}

pub struct Force {
    pub force: Vec3
}

crate::resource!{
    #[uuid = "11c82e72-b7b5-433f-8fa1-440796c714aa"]
    struct BlackBody {
        temperature: f32
    }
}

pub fn gravity(
    thread_pool: Res<ComputeTaskPool>,
    mut set: QuerySet<(
        Query<(&Mass, &RigidBodyPosition, &Identity)>,
        Query<(&mut Force, &Mass, &RigidBodyPosition, &Identity)>
    )>
) {
    // q.for_each_mut(|(m)| {
    //     info!("m.mass: {}", m.mass);
    //     // if m.mass > crate::constants::GRAVITY_MIN_MASS {
    //     //     let (v3, _): (Vec3, Quat) = cb.position.into();
    //     //     heavy.push((m.mass, v3, e.id()));
    //     // }
    // });
    let mut heavy = Vec::new();
    set.q0().for_each_mut(|(m, cb, e)| {
        info!("m.mass: {}", m.mass);
        if m.mass > crate::constants::GRAVITY_MIN_MASS {
            let (v3, _): (Vec3, Quat) = cb.position.into();
            heavy.push((m.mass, v3, e.id));
        }
    });

    set.q1_mut().par_for_each_mut(&thread_pool, num_cpus::get(), |(mut f2, m2, cb2, e2)| {
        println!("id: {:?}", e2.id);
        heavy.clone().into_iter().for_each(|(m1, t1, e1)| {
            let mass_prod = m1 * m2.mass;
            if e1 != e2.id && mass_prod > crate::constants::GRAVITY_MIN_MASS_PROD {
                let (pos, _) = cb2.position.into();
                let r2 = t1.distance_squared(pos) as f64;

                if r2 >= crate::constants::GRAVITY_MIN_DISTANCE && r2 <= crate::constants::GRAVITY_MAX_DISTANCE {
                    let m = (mass_prod * crate::constants::PHYSICS_GRAVITY / r2) as f32;
                    let d = (t1 - pos).normalize();                    

                    // f2.is_dirty = true;
                    f2.force += m * d;
                }
            }
        });
    });
}

// #[derive(Debug, Default)]
// pub struct Impulse {
//     pub impulse: Vec3,
// }

// // impl ImpulseEvent {
// //     pub fn new(other: &Vec3) -> Self {
// //         Self { impulse: *other }
// //     }
// // }

// impl Deref for Impulse {
//     type Target = Vec3;

//     fn deref(&self) -> &Self::Target {
//         &self.impulse
//     }
// }


pub fn impulse(
    time: Res<Time>,
    thread_pool: Res<ComputeTaskPool>,
    // mut impulses: EventReader<ImpulseEvent>,
    mut query: Query<(&mut RigidBodyVelocity, &mut RigidBodyActivation, &RigidBodyMassProps, &mut Force), (With<RigidBodyVelocity>, With<RigidBodyActivation>, With<RigidBodyMassProps>, With<Force>)>,
    // thread_pool: Res<ComputeTaskPool>,
) {
    // let mut impulse = Vec3::ZERO;
    // for event in impulses.iter() {
    //     impulse += **event;
    // }
    // if impulse.length_squared() > 1E-6 {
        query.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut rbv, mut rba, rbm, mut f)| {
            let impulse = f.force * time.delta_seconds();
            rbv.apply_impulse(rbm, impulse.into());
            rba.wake_up(true);

            f.force = Vec3::ZERO;
        });
    // }
}

#[derive(Debug, Default)]
pub struct ForceEvent {
    force: Vec3,
}

impl ForceEvent {
    pub fn new(other: &Vec3) -> Self {
        Self { force: *other }
    }
}

impl Deref for ForceEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.force
    }
}

// pub struct Heat {
//     pub temperature: f64
// }

// pub fn black_body(
//     mut heat_query: Query<(&Heat), (With<Heat>)>,
// ) {
//     for (h) in heat_query.iter_mut() {
//         if h.temperature > 3000.0 {
//             let e = plancks_law_rgb(h.temperature);
//         }
//     }
// }

// static PLANCK_CONSTANT: f64 = 6.62607015e-34;
// static SPEED_OF_LIGHT: f64 = 299792458.0;
// static BOLTZMANN_CONSTANT: f64 =  1.380649e-23;
// pub fn plancks_law_rgb(t: f64) -> Color {
//     let r = plancks_law(4.62e14, t) as f32;
//     let g = plancks_law(5.45e14, t) as f32;
//     let b = plancks_law(6.66e14, t) as f32;

//     return Color::rgb(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
// }
// pub fn plancks_law(f: f64, t: f64) -> f64 {
//     let top = 2.0 * PLANCK_CONSTANT * f.powi(3) / SPEED_OF_LIGHT.powi(2);
//     let bottom = (PLANCK_CONSTANT*f/(BOLTZMANN_CONSTANT * t)).exp() - 1.0;
//     let res = top / bottom ;

//     return res;
// }