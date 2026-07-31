use std::collections::HashMap;
use std::env;
use std::time::Instant;

fn main() {
    let filename = env::args().nth(1).expect("usage: ground_grid <file.laz>");
    let cell_size = 1.0_f64;
    let height_threshold = 0.3_f64;

    let t0 = Instant::now();

    let mut reader = las::Reader::from_path(&filename).expect("failed to open file");
    let point_data = reader.read_all().expect("failed to read points");

    let x: Vec<f64> = point_data.x().collect();
    let y: Vec<f64> = point_data.y().collect();
    let z: Vec<f64> = point_data.z().collect();
    let n = x.len();

    let load_time = t0.elapsed();
    let t1 = Instant::now();

    let xmin = x.iter().cloned().fold(f64::INFINITY, f64::min);
    let ymin = y.iter().cloned().fold(f64::INFINITY, f64::min);

    let mut cell_id = vec![0i64; n];
    let mut min_z_per_cell: HashMap<i64, f64> = HashMap::with_capacity(n / 20);

    for i in 0..n {
        let ix = ((x[i] - xmin) / cell_size).floor() as i64;
        let iy = ((y[i] - ymin) / cell_size).floor() as i64;
        let id = (ix << 32) ^ (iy & 0xffffffff);
        cell_id[i] = id;
        min_z_per_cell
            .entry(id)
            .and_modify(|m| { if z[i] < *m { *m = z[i]; } })
            .or_insert(z[i]);
    }

    let mut ground_count = 0i64;
    for i in 0..n {
        let ground_z = min_z_per_cell[&cell_id[i]];
        if z[i] - ground_z < height_threshold {
            ground_count += 1;
        }
    }

    let process_time = t1.elapsed();

    println!("total points: {}", n);
    println!("grid cells: {}", min_z_per_cell.len());
    println!(
        "ground points: {} ({:.1}%)",
        ground_count,
        100.0 * ground_count as f64 / n as f64
    );
    eprintln!("load: {:?}", load_time);
    eprintln!("process (grid ground filter): {:?}", process_time);
}
