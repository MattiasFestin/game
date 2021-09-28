use std::ops::{Add, Mul};

use core_simd::*;

const bit_noise1: u64 = 0xB5297A4D;
const bit_noise2: u64 = 0x68E31DA4;
const bit_noise3: u64 = 0x1B56C4E9;
fn squirrel3(position: u64, seed: u64) -> u64 {
    let mut mangled = position;
    mangled *= bit_noise1;
    mangled += seed;
    mangled ^= (mangled >> 8);
    mangled += bit_noise2;
    mangled ^= (mangled << 8);
    mangled *= bit_noise3;
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

const offset_2d: u64x8 = u64x8::from_array([
    0u64,           1u64,           2u64,           3u64, 
    0u64 + prime_y, 1u64 + prime_y, 2u64 + prime_y, 3u64 + prime_y
]);
pub fn noise_2d_simd(x: u64, y: u64, seed: u64) -> [u64; 8] {
    let position = u64x8::splat(x).add(offset_2d);
    return squirrel3_simd(position, seed);
}

const offset_3d: u64x8 = u64x8::from_array([
    0u64,                       1u64,
    0u64 + prime_y,             1u64 + prime_y, 
    0u64 + prime_z,             1u64 + prime_z,
    0u64 + prime_y.wrapping_add(prime_z),   1u64 + prime_y.wrapping_add(prime_z)
]);
pub fn noise_3d_simd(x: u64, y: u64, z: u64, seed: u64) -> [u64; 8] {
    let position = u64x8::splat(x).add(offset_2d);
    return squirrel3_simd(position, seed);
}