#[path = "./day03/mod.rs"]
mod day;

fn main() {
    let start = std::time::Instant::now();
    day::part1();
    day::part2();
    println!("Finished in {:?}", start.elapsed());
}
