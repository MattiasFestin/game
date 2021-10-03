use bevy::prelude::*;


/*pub fn gravity(
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
}*/

pub struct Heat {
    pub temperature: f64
}

pub fn black_body(
    mut heat_query: Query<(&Heat), (With<Heat>)>,
) {
    for (h) in heat_query.iter_mut() {
        if h.temperature > 3000.0 {
            let e = plancks_law_rgb(h.temperature);
            println!("emissive: {:?}", e);
        }
    }
}

static PLANCK_CONSTANT: f64 = 6.62607015e-34;
static SPEED_OF_LIGHT: f64 = 299792458.0;
static BOLTZMANN_CONSTANT: f64 =  1.380649e-23;
pub fn plancks_law_rgb(t: f64) -> Color {
    let r = 5310339.90294 * plancks_law(4.62e14, t) as f32;
    let g = 5310339.90294 * plancks_law(5.45e14, t) as f32;
    let b = 5310339.90294 * plancks_law(6.66e14, t) as f32;

    return Color::rgb(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
}
pub fn plancks_law(f: f64, t: f64) -> f64 {
    let top = 2.0 * PLANCK_CONSTANT * f.powi(3) / SPEED_OF_LIGHT.powi(2);
    let bottom = (PLANCK_CONSTANT*f/(BOLTZMANN_CONSTANT * t)).exp() - 1.0;
    let res = top / bottom ;

    return res;
}