//! # Noise
//! Implementations of noise algorithms.

/// A very fast Perlin noise implementation.
/// Returns values in [0, 1].
#[inline]
pub fn perlin_noise(x: f32, y: f32, lengthscale: f32, seed: u32) -> f32 {
    // Scale coordinates by lengthscale.
    let scaled_x = x / lengthscale;
    let scaled_y = y / lengthscale;

    // Find grid cell.
    let x0 = scaled_x.floor() as i32;
    let y0 = scaled_y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    // Get fractional part within cell.
    let fractional_x = scaled_x - x0 as f32;
    let fractional_y = scaled_y - y0 as f32;

    // Get gradients at four corners.
    let grad_00 = gradient_2d(x0, y0, seed);
    let grad_10 = gradient_2d(x1, y0, seed);
    let grad_01 = gradient_2d(x0, y1, seed);
    let grad_11 = gradient_2d(x1, y1, seed);

    // Calculate dot products with distance vectors.
    let d00 = dot_2d(grad_00, (fractional_x, fractional_y));
    let d10 = dot_2d(grad_10, (fractional_x - 1.0, fractional_y));
    let d01 = dot_2d(grad_01, (fractional_x, fractional_y - 1.0));
    let d11 = dot_2d(grad_11, (fractional_x - 1.0, fractional_y - 1.0));

    // Cubic interpolation.
    let u = cerp(fractional_x);
    let v = cerp(fractional_y);

    // Bilinear interpolation.
    let nx0 = lerp(d00, d10, u);
    let nx1 = lerp(d01, d11, u);
    let nxy = lerp(nx0, nx1, v);

    // Map from [-1, 1] to [0, 1].
    (nxy + 1.0) * 0.5
}

/// Very fast hash function for generating gradients.
#[inline]
fn hash_2d(x: i32, y: i32, seed: u32) -> u32 {
    let mut hash = (x as u32).wrapping_mul(374761393);
    hash = hash.wrapping_add(y as u32).wrapping_mul(668265263);
    hash = hash.wrapping_add(seed).wrapping_mul(1274126177);
    hash ^= hash >> 16;
    hash.wrapping_mul(2246822519)
}

/// Generate unit gradient vector from grid coordinates.
#[inline]
fn gradient_2d(x: i32, y: i32, seed: u32) -> (f32, f32) {
    let h = hash_2d(x, y, seed);

    // Use Ken Perlin's gradient selection approach for better performance.
    // This gives 8 evenly distributed gradients.
    match h & 7 {
        0 => (1.0, 1.0),
        1 => (-1.0, 1.0),
        2 => (1.0, -1.0),
        3 => (-1.0, -1.0),
        4 => (1.0, 0.0),
        5 => (-1.0, 0.0),
        6 => (0.0, 1.0),
        _ => (0.0, -1.0),
    }
}

/// 2D dot product.
#[inline]
fn dot_2d(a: (f32, f32), b: (f32, f32)) -> f32 {
    a.0 * b.0 + a.1 * b.1
}

/// Cubic smoothstep interpolation.
#[inline]
fn cerp(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Linear interpolation.
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_range() {
        // Test that output is always in [0, 1].
        for i in 0..1000 {
            let x = (i as f32) * 0.1;
            let y = (i as f32) * 0.1;
            let noise = perlin_noise(x, y, 10.0, 42);
            assert!((0.0..=1.0).contains(&noise));
        }
    }

    #[test]
    fn test_deterministic() {
        // Same inputs should give same outputs.
        let noise1 = perlin_noise(5.0, 3.0, 2.0, 123);
        let noise2 = perlin_noise(5.0, 3.0, 2.0, 123);
        assert_eq!(noise1, noise2);
    }

    #[test]
    fn test_different_seeds() {
        // Different seeds should give different results.
        let noise1 = perlin_noise(5.0, 3.0, 2.0, 123);
        let noise2 = perlin_noise(5.0, 3.0, 2.0, 456);
        assert_ne!(noise1, noise2);
    }

    #[test]
    fn test_lengthscale_effect() {
        // Larger lengthscale should give smoother variation.
        let noise1 = perlin_noise(5.0, 3.0, 1.0, 123);
        let noise2 = perlin_noise(5.1, 3.0, 1.0, 123);
        let noise3 = perlin_noise(5.0, 3.0, 10.0, 123);
        let noise4 = perlin_noise(5.1, 3.0, 10.0, 123);

        // Difference should be smaller with larger lengthscale.
        let diff_small = (noise2 - noise1).abs();
        let diff_large = (noise4 - noise3).abs();
        assert!(diff_large < diff_small);
    }
}
