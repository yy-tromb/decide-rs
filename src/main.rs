use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    match std::env::args().skip(1).next(){
        Some(by) => {
            let by: i64 = by.parse().expect("Failed to parse as int");
            let r1: f64 = rng.random();
            println!("{}",r1*(by as f64));
        },
        None => {
            let r1: f64 = rng.random();
            println!("{}",r1);
        }
    };
}
