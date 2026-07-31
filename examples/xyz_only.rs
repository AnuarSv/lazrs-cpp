use std::env;
use std::time::Instant;

fn main() {
    let filename = env::args().nth(1).expect("usage: xyz_only <file.laz>");
    let t0 = Instant::now();

    let mut reader = las::Reader::from_path(&filename).expect("failed to open file");
    let point_data = reader.read_all().expect("failed to read points");

    let x: Vec<f64> = point_data.x().collect();
    let y: Vec<f64> = point_data.y().collect();
    let z: Vec<f64> = point_data.z().collect();

    let load_time = t0.elapsed();
    println!("total points: {}", x.len());
    eprintln!("load: {:?}", load_time);
    std::hint::black_box((y, z));
}
