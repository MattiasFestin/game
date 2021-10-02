use bevy::math::Vec3;

struct Spherical {
    pub r: f32,
    pub φ: f32,
    pub θ: f32,
}

impl From<Vec3> for Spherical {
    fn from(p: Vec3) -> Self {
        let r = p.length();
        Self {
            r,
            θ: (p.z / r).acos(),
            φ: (p.x/p.y).atan() + if p.x < 0.0 { std::f32::consts::PI } else { 0.0 }
        }
    }
}

impl From<Spherical> for Vec3 {
    fn from(s: Spherical) -> Self {
        Vec3::new(
            s.r * s.φ.cos() * s.θ.sin(),
            s.r * s.φ.sin() * s.θ.sin(),
            s.r * s.θ.cos()
        )
    }
}