use std::f32;

use noise::NoiseFn;

pub fn noise_2d<T>(
    noise_fn: &T,
    x: usize,
    y: usize,
    octaves: u8,
    frequency: f64,
    lacunarity: f64,
) -> f32
where
    T: NoiseFn<f64, 2>,
{
    let mut total = 0.0;
    let mut frequency = frequency;
    let mut amplitude = 1.0;
    let mut max_amplitude = 0.0;

    for _ in 0..octaves {
        let val = noise_fn.get([x as f64 * frequency, y as f64 * frequency]);

        total += val * amplitude;
        max_amplitude += amplitude;
        frequency *= lacunarity;
        amplitude /= 2.0;
    }

    total as f32 / max_amplitude as f32
}

pub fn noise_3d<T>(
    perlin: &T,
    x: usize,
    y: usize,
    z: usize,
    octaves: u8,
    frequency: f64,
    lacunarity: f64,
) -> f32
where
    T: NoiseFn<f64, 3>,
{
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
