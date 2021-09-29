use std::ops::{Add, Mul};

use core_simd::*;

const bit_noise1: u64 = 0xB5297A4D;
const bit_noise2: u64 = 0x68E31DA4;
const bit_noise3: u64 = 0x1B56C4E9;
fn squirrel3(position: u64, seed: u64) -> u64 {
    let mut mangled = position;
    mangled = mangled.wrapping_mul(bit_noise1);
    mangled = mangled.wrapping_add(seed);
    mangled ^= (mangled >> 8);
    mangled = mangled.wrapping_add(bit_noise2);
    mangled ^= (mangled << 8);
    mangled = mangled.wrapping_mul(bit_noise3);
    mangled ^= (mangled >> 8);
    return mangled;
}

pub fn noise_1d(x: u64, seed: u64) -> u64 {
    return squirrel3(x, seed);
}

const prime_y: u64 = 14536142487739796659;
const prime_z: u64 = 17330241684369242527;
pub fn noise_2d(x: u64, y: u64, seed: u64) -> u64 {
    return squirrel3(x.wrapping_add(y.wrapping_mul(prime_y)), seed);
}

pub fn noise_3d(x: u64, y: u64, z: u64, seed: u64) -> u64 {
    return squirrel3(x.wrapping_add(y.wrapping_mul(prime_y)).wrapping_add(z.wrapping_mul(prime_z)), seed);
}

const bit_noise_simd1: u64x8 = u64x8::splat(0xB5297A4D);
const bit_noise_simd2: u64x8 = u64x8::splat(0x68E31DA4);
const bit_noise_simd3: u64x8 = u64x8::splat(0x1B56C4E9);
fn squirrel3_simd(position: u64x8, seed: u64) -> [u64; 8] {
    let mut mangled = position;
    mangled *= bit_noise_simd1;
    mangled += seed;
    mangled ^= (mangled >> 8);
    mangled += bit_noise_simd2;
    mangled ^= (mangled << 8);
    mangled *= bit_noise_simd3;
    mangled ^= (mangled >> 8);
    return mangled.to_array();
}

const offset_1d: u64x8 = u64x8::from_array([0u64, 1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64]);
pub fn noise_1d_simd(x: u64, seed: u64) -> [u64; 8] {
    let position = u64x8::splat(x).add(offset_1d);
    return squirrel3_simd(position, seed);
}

pub fn noise_2d_simd(x: u64, y: u64, seed: u64) -> [u64; 8] {
    return noise_1d_simd(x.wrapping_add(y.wrapping_mul(prime_y)), seed);
}

const OFFSET_3D: u64x8 = u64x8::from_array([
    0u64,                       1u64,
    0u64 + prime_y,             1u64 + prime_y, 
    0u64 + prime_z,             1u64 + prime_z,
    0u64 + prime_y.wrapping_add(prime_z),   1u64 + prime_y.wrapping_add(prime_z)
]);
pub fn noise_3d_simd(x: u64, y: u64, z: u64, seed: u64) -> [u64; 8] {
    let position = u64x8::splat(x).add(offset_3d);
    return squirrel3_simd(position, seed);
}

pub fn perlin_2d(width: usize, height: usize, seed: u64) {
    let mut pixels = Vec::new();
    for x in 1..=width {
        for y in 1..=height/8 {
            //Noise2D generally returns a value in the range [-1.0, 1.0]
            let n = Vec::from(noise_2d_simd(y as u64, x as u64, seed));
            pixels.push(n);
            // t n = Noise2D(x*0.01, y*0.01);
            
            // //Transform the range to [0.0, 1.0], supposing that the range of Noise2D is [-1.0, 1.0]
            // n += 1.0;
            // n /= 2.0;
            
            // int c = Math.round(255*n);
            // pixels[y][x] = new Color(c, c, c);
        }
    }
    // for(int y = 0; y < 500; y++){
    //     for(int x = 0; x < 500; x++){
    //         //Noise2D generally returns a value in the range [-1.0, 1.0]
    //         float n = Noise2D(x*0.01, y*0.01);
            
    //         //Transform the range to [0.0, 1.0], supposing that the range of Noise2D is [-1.0, 1.0]
    //         n += 1.0;
    //         n /= 2.0;
            
    //         int c = Math.round(255*n);
    //         pixels[y][x] = new Color(c, c, c);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn squirrel3_tests() {
        let seed = 55u64;
        assert_eq!(12687927802791220436, crate::noise::squirrel3(1, seed));

        let seed = 56u64;
        assert_eq!(12687927848928216793, crate::noise::squirrel3(1, seed));

        let seed = 0u64;
        assert_eq!(3033592379929695938, crate::noise::squirrel3(0, seed));
    }

    #[test]
    fn squirrel3_simd_tests() {
        let seed = 55u64;
        let res = crate::noise::noise_1d_simd(0, seed);
        assert_eq!(crate::noise::squirrel3(0, seed), res[0]);
        assert_eq!(crate::noise::squirrel3(1, seed), res[1]);
        assert_eq!(crate::noise::squirrel3(2, seed), res[2]);
        assert_eq!(crate::noise::squirrel3(3, seed), res[3]);
        assert_eq!(crate::noise::squirrel3(4, seed), res[4]);
        assert_eq!(crate::noise::squirrel3(5, seed), res[5]);
        assert_eq!(crate::noise::squirrel3(6, seed), res[6]);
        assert_eq!(crate::noise::squirrel3(7, seed), res[7]);
    }
}
