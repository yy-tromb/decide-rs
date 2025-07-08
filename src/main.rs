use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let r1 : f64 = rng.random();
    println!("{}",r1);
}
