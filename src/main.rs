use image::ImageBuffer;
use rand::Rng;

struct Noise {
    values: Vec<f64>,
    width: i32,
    height: i32,
}

impl Noise {
    pub fn set(&mut self, x: i32, y: i32, value: f64) {
        self.values[(self.height * x + y) as usize] = value;
    }

    pub fn get(&self, x: i32, y: i32) -> f64 {
        if x < 0 || y < 0 || self.width <= x || self.height <= y {
            0.5f64
        } else {
            self.values[(x * self.height + y) as usize]
        }
    }
}

fn static_noise(width: i32, height: i32) -> Noise {
    let mut rng = rand::thread_rng();
    let mut values: Vec<f64> = vec![];
    for _x in 0..width {
        for _y in 0..height {
            values.push(rng.gen());
        }
    }
    Noise {
        values: values,
        width: width,
        height: height,
    }
}

fn padded_noise(width: i32, height: i32, padding: i32) -> Noise {
    let mut rng = rand::thread_rng();
    let mut values: Vec<f64> = vec![];
    // initial pass setting key points
    for x in 0..width {
        for y in 0..height {
            if x % padding == 0 && y % padding == 0 {
                values.push(rng.gen());
            } else {
                values.push(0f64);
            }
        }
    }
    Noise {
        values: values,
        width: width,
        height: height,
    }
}

fn lattice_noise(width: i32, height: i32) -> Noise {
    let mut rng = rand::thread_rng();
    let mut values = vec![];
    for x in 0..width {
        for y in 0..height {
            if (x + y) % 2 == 0 {
                values.push(rng.gen());
            } else {
                values.push(0f64);
            }
        }
    }
    Noise {
        values: values,
        width: width,
        height: height,
    }
}

fn smooth_lattice_noise(width: i32, height: i32) -> Noise {
    let mut noise = lattice_noise(width, height);
    for x in 0..width {
        for y in 0..height {
            if (x + y) % 2 == 0 {
                continue;
            }
            let sum = noise.get(x - 1, y)
                + noise.get(x + 1, y)
                + noise.get(x, y - 1)
                + noise.get(x, y + 1);
            noise.set(x, y, sum / 4f64);
        }
    }
    noise
}

fn brightness(value: f64) -> u8 {
    (value * 255f64) as u8
}

fn main() {
    // let noise: Noise = padded_noise(256, 256, 4);
    let noise: Noise = smooth_lattice_noise(256, 256);
    let img = ImageBuffer::from_fn(256, 256, |x, y| {
        image::Luma([brightness(noise.get(x as i32, y as i32))])
    });
    img.save("static.png").unwrap();
}
