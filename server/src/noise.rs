use perlin2d::PerlinNoise2D;

fn gen_noise(x: i64, y: i64, perlin: &PerlinNoise2D) -> f32 {
    perlin.get_noise(x as f64, y as f64) as f32
}


pub fn get_map(x: i64, y: i64, seed: i32) -> Vec<f32> {
    let perlin = get_perlin(seed);
    let mut out = vec![0.; (x * y) as usize];
    
    for i in 0..x {
        for j in 0..y {
           let noise = gen_noise(i, j, &perlin);
           let index = (i + j*y) as usize;
           out[index] = noise;
        }
    }

    return out
}


fn get_perlin( seed: i32) -> PerlinNoise2D {
    PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, seed)
}
