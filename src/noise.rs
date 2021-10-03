const BIT_NOISE1: u64 = 0xB5297A4D;
const BIT_NOISE2: u64 = 0x68E31DA4;
const BIT_NOISE3: u64 = 0x1B56C4E9;
fn squirrel3(position: u64, seed: u64) -> u64 {
    let mut mangled = position;
    mangled = mangled.wrapping_mul(BIT_NOISE1);
    mangled = mangled.wrapping_add(seed);
    mangled ^= mangled >> 8;
    mangled = mangled.wrapping_add(BIT_NOISE2);
    mangled ^= mangled << 8;
    mangled = mangled.wrapping_mul(BIT_NOISE3);
    mangled ^= mangled >> 8;
    return mangled;
}

pub fn noise_1d(x: u64, seed: u64) -> u64 {
    return squirrel3(x, seed);
}

const PRIME_Y: u64 = 14536142487739796659;
const PRIME_Z: u64 = 17330241684369242527;
pub fn noise_2d(x: u64, y: u64, seed: u64) -> u64 {
    return squirrel3(x.wrapping_add(y.wrapping_mul(PRIME_Y)), seed);
}

pub fn noise_3d(x: u64, y: u64, z: u64, seed: u64) -> u64 {
    return squirrel3(x.wrapping_add(y.wrapping_mul(PRIME_Y)).wrapping_add(z.wrapping_mul(PRIME_Z)), seed);
}

pub fn noise_1d_f64_normalized(x: f64, seed: u64) -> f64 {
    return (squirrel3(x as u64, seed) as f64) / (u64::MAX as f64);
}

pub fn noise_3d_f64_normalized(x: u64, y: u64, z: u64, seed: u64) -> f64 {
    return (noise_3d(x, y, z, seed) as f64) / (u64::MAX as f64);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;


    #[test]
    fn squirrel3_tests() {
        let seed = 55u64;
        assert_eq!(12687927802791220436, crate::noise::squirrel3(1, seed));

        let seed = 56u64;
        assert_eq!(12687927848928216793, crate::noise::squirrel3(1, seed));

        let seed = 0u64;
        assert_eq!(3033592379929695938, crate::noise::squirrel3(0, seed));
    }

    #[bench]
    fn noise_1d(b: &mut Bencher) {
        let mut seed = test::black_box(1000);
        b.iter(|| {
            for _ in 1..=128 {
                
                let y= [
                    crate::noise::noise_1d(12687927802791220436, seed),
                    crate::noise::noise_1d(12687927802791220437, seed),
                    crate::noise::noise_1d(12687927802791220438, seed),
                    crate::noise::noise_1d(12687927802791220439, seed),
                    crate::noise::noise_1d(12687927802791220440, seed),
                    crate::noise::noise_1d(12687927802791220441, seed),
                    crate::noise::noise_1d(12687927802791220442, seed),
                    crate::noise::noise_1d(12687927802791220443, seed),
                ];
                seed = y[0];
            }
        });
    }

    #[bench]
    fn perlin_2d(b: &mut Bencher) {
        let seed = test::black_box(1000);
        b.iter(|| {
            simdnoise::NoiseBuilder::fbm_2d(1024, 1024).with_seed(seed).generate_scaled(0.0, 1.0);
        });
    }
}
