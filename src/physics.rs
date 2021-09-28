use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Mul;
use std::rc::Rc;

use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::tasks::{ComputeTaskPool};
use bevy_frustum_culling::aabb::Aabb;
// use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use rayon::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct Physics;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsSystem {
    Position,
    Velocity,
    Force,
    Gravity,
}

pub struct Identity {
    pub id: u64
}

pub struct Position {
    pub position: Vec3A,
}

pub fn position(
    thread_pool: Res<ComputeTaskPool>,
    mut query: Query<(&mut Transform, &mut Position), Changed<Position>>
) {
    query.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut transform, mut p)| {
        transform.translation = p.position.into();
    });
}

pub struct Velocity {
    pub velocity: Vec3A,
}

pub fn velocity(
    time: Res<Time>,
    thread_pool: Res<ComputeTaskPool>,
    mut query: Query<(&mut Position, &mut Velocity), (With<Position>, With<Velocity>)>
) {
    query.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut p, mut v)| {
        if v.velocity.length() > crate::constants::MIN_VELOCITY {
            p.position += v.velocity * time.delta_seconds();
        }
    });
}

pub struct Mass {
    pub mass: f32
}

pub struct Force {
    pub force: Vec3A,
    pub is_dirty: bool,
}

pub fn force(
    time: Res<Time>,
    thread_pool: Res<ComputeTaskPool>,
    mut query: Query<(&mut Velocity, &mut Force, &Mass), (With<Force>, With<Velocity>, With<Mass>)>
) {
    query.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut v, mut f, m)| {
        if f.force.length() > crate::constants::MIN_FORCE && m.mass < crate::constants::MAX_MASS && v.velocity.length() < crate::constants::MAX_SPEED {
            v.velocity += (f.force / m.mass) * time.delta_seconds();
        }
    });
}

pub fn gravity(
    time: Res<Time>,
    thread_pool: Res<ComputeTaskPool>,
    // mut query: Query<(&mut &Force, &Mass, &Position), (With<Force>, With<Mass>, With<Position>)>
    mut set: QuerySet<(
        Query<(&Mass, &Position, &Identity), (With<Mass>, With<Position>, With<Identity>)>,
        Query<(&mut Force, &mut Mass, &Position, &Identity), (With<Force>, With<Mass>, With<Position>, With<Identity>)>
    )>
) {
    let mut heavy = Vec::new();
    {
        set.q0().for_each_mut(|(m,p, i)| {
            if m.mass > crate::constants::GRAVITY_MIN_MASS {
                heavy.push((m.mass, p.position.clone(), i.id));
            }
        });
    }

    set.q1_mut().par_for_each_mut(&thread_pool, num_cpus::get(), |(mut f2, mut m2, p2, i2)| {
        heavy.clone().into_iter().for_each(|(m1, p1, i1)| {
            let mass_prod = m1 * m2.mass;
            if i1 != i2.id && mass_prod > crate::constants::GRAVITY_MIN_MASS_PROD {
                let r2 = p1.distance_squared(p2.position);

                if r2 >= crate::constants::GRAVITY_MIN_DISTANCE && r2 <= crate::constants::GRAVITY_MAX_DISTANCE {
                    let m = mass_prod * crate::constants::PHYSICS_GRAVITY / r2;
                    let d = (p1 - p2.position).normalize();

                    

                    f2.is_dirty = true;
                    f2.force += m * d * time.delta_seconds();
                }
            }
        });
    });
}

pub struct InelasticCollision {
    pub cor: f32 // coefficient of restitution
}

pub fn collition(
    thread_pool: Res<ComputeTaskPool>,
    mut set: QuerySet<(
        Query<(&Velocity, &Mass, &Position, &Identity, &InelasticCollision), (With<Mass>, With<Velocity>, With<Position>, With<Identity>, With<InelasticCollision>)>,
        Query<(&mut Velocity, &Mass, &Position, &Identity, &InelasticCollision), (With<Velocity>, With<Position>, With<Identity>, With<InelasticCollision>)>,
        // Query<(&bevy_frustum_culling::aabb::Aabb, &Identity), (With<bevy_frustum_culling::aabb::Aabb>, With<Identity>)>,
        // Query<(&mut Position, &bevy_frustum_culling::aabb::Aabb, &Identity), (&mut Position, With<bevy_frustum_culling::aabb::Aabb>, With<Identity>)>
    )>
) {
    let mut coliders = Vec::new();
    {
        set.q0().for_each_mut(|(v, m, p, i, ic)| {
            coliders.push((v.velocity, m.mass, p.position.clone(), i.id, ic.cor));
        });
    }

    set.q1_mut().par_for_each_mut(&thread_pool, num_cpus::get(), |(mut v2, m2, p2, i2, ic2)| {
        coliders.clone().into_iter().for_each(|(v1, m1, p1, i1, ic1)| {
            if i1 != i2.id && p1.distance(p2.position) < 1.0 {
                let u1 = m1 * v1;
                let u2 = m2.mass * v2.velocity;
                let mt = m1 + m2.mass;
                let pd = p1-p2.position;
                let dot = (v2.velocity-v1).dot(pd);
                let ic = ic1 * ic2.cor;

                //TEST
                v2.velocity -=  ic * 2.0*m1 / mt * dot/p1.distance_squared(p2.position) * pd;
            }
        });
    });
}