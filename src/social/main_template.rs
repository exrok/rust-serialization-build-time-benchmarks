#[allow(unused_imports, dead_code)]
mod models;
mod compat;
mod db;
mod routes;

fn main() {
    let mut args = std::env::args();
    args.next();
    let db_path = args.next().unwrap();
    let route = args.next().unwrap();
    let repeat: u64 = args.next().unwrap().parse().unwrap();

    let mut input = String::new();
    use std::io::Read;
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut last_output = String::new();
    for _ in 0..repeat {
        std::hint::black_box(&mut input);
        last_output = routes::dispatch(&db_path, &route, &input);
        std::hint::black_box(&last_output);
    }
    print!("{}", last_output);
}
