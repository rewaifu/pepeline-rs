use std::f32;

use noise::{NoiseFn, Perlin};

pub fn perlin_noise_2d(
    perlin: &Perlin,
    x: usize,
    y: usize,
    octaves: u8,
    frequency: f64,
    lacunarity: f64,
) -> f32 {
    let mut total = 0.0;
    let mut frequency = frequency;
    let mut amplitude = 1.0;
    let mut max_amplitude = 0.0;

    for _ in 0..octaves {
        let val = perlin.get([x as f64 * frequency, y as f64 * frequency]);

        total += val * amplitude;
        max_amplitude += amplitude;
        frequency *= lacunarity;
        amplitude /= 2.0;
    }

    total as f32 / max_amplitude as f32
}
pub fn perlin_noise_3d(
    perlin: &Perlin,
    x: usize,
    y: usize,
    z: usize,
    octaves: u8,
    frequency: f64,
    lacunarity: f64,
) -> f32 {
    let mut total = 0.0;
    let mut frequency = frequency;
    let mut amplitude = 1.0;
    let mut max_amplitude = 0.0;

    for _ in 0..octaves {
        let val = perlin.get([
            x as f64 * frequency,
            y as f64 * frequency,
            z as f64 * frequency,
        ]);

        total += val * amplitude;
        max_amplitude += amplitude;
        frequency *= lacunarity;
        amplitude /= 2.0;
    }

    total as f32 / max_amplitude as f32
}
