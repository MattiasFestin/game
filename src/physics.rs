use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Mul;
use std::rc::Rc;

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, ComputeTaskPool};
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
    pub position: Vec3,
}
pub fn position(
    thread_pool: Res<AsyncComputeTaskPool>,
    mut query: Query<(&mut Transform, &mut Position), Changed<Position>>
) {
    query.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut transform, mut p)| {
        transform.translation = p.position;
    });
}

pub struct Velocity {
    pub velocity: Vec3,
}

pub fn velocity(
    time: Res<Time>,
    thread_pool: Res<AsyncComputeTaskPool>,
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
    pub force: Vec3,
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
    thread_pool: Res<AsyncComputeTaskPool>,
    // mut query: Query<(&mut &Force, &Mass, &Position), (With<Force>, With<Mass>, With<Position>)>
    mut set: QuerySet<(
        Query<(&Mass, &Position, &Identity), (With<Mass>, With<Position>, With<Identity>)>,
        Query<(&mut Force, &mut Mass, &Position, &Identity), (With<Force>, With<Mass>, With<Position>, With<Identity>)>
    )>
) {
    let mut heavy = Vec::new();
    // let mut b = Rc::new(RefCell::new(Vec::new()));
    {
        set.q0().for_each_mut(|(m,p, i)| {
            if m.mass > crate::constants::GRAVITY_MIN_MASS {
                heavy.push((m.mass, p.position.clone(), i.id));
            }
        });
    }

    
//     // let mut b = Vec::new();
    // set.q1_mut().for_each_mut(|(mut f1, m2, p2, i2)| {
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
                    // m2.mass *= 1.01;
                }
                // println!("{:?} => {:?} * {:?} * {:?} / {:?} = {:?}", i2.id, m1, m2.mass, crate::constants::PHYSICS_GRAVITY, r2, m);
            }
        });
    });
    
    // for (m1, p1, i1) in {
    //     // let (m1, p1, i1) = &*el.borrow_mut();
    //     // for el2 in a.into_iter() {

    //     // }
    // }
    // for (mut f1, m1, p1, i1) in query.iter_mut() {
    //     let b: Vec<Rc<RefCell<(Mut<Force>, &Mass, &Position, &Identity)>> = a.borrow();
    //     for (mut f2, m2, p2, i2) in b.into_iter() {

    //     }
    // }
    // for (m2, p2, i2) in set.q1().iter() {
//             if i1.id != i2.id && m1.mass + m2.mass > crate::constants::MIN_MASS && m1.mass > m2.mass {
//                 println!("Gravity: ({:?}, {:?})", i1.id, i2.id);
//                 let r2 = p1.position.distance_squared(p2.position);
//                 let m = m1.mass * m2.mass * crate::constants::PHYSICS_GRAVITY / r2;
//                 let d = (p1.position - p2.position).normalize();

//                 f1.is_dirty = true;
//                 f1.force = m * d;

//                 // f2.is_dirty = true;
//                 // f2.force = -m * d;
//             }
//         }
//     }
    // query.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut f1, m1, p1, i1)| {
        
    // });
//     //     // q2.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut f2, m2, p2, i2)| {
//     //     //     if i1.id != i2.id && m1.mass + m2.mass > crate::constants::MIN_MASS {
                
//     //     //     }
//     //     // });
//     //     // q2;
//     // });

//     // a.par_iter().enumerate().for_each(|(i, (mut f1, m1, p1))| {
//     //     b.par_iter().enumerate().for_each(|(j, (mut f2, m2, p2))| {
//     //         if i != j && m1.mass + m2.mass > crate::constants::MIN_MASS {
                
//     //         }
//     //     });
//     // });
}