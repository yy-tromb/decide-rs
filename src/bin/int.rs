use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let r1 :f64 = rng.random();
    let by :i64 = std::env::args().skip(1).next().unwrap().parse().unwrap();
    println!("{}",r1*(by as f64));
}
