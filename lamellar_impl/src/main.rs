use lamellar::LamellarWorldBuilder;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let world = LamellarWorldBuilder::new().build();
    // let start = std::time::Instant::now();
    // day1::part_1(&world);
    // println!("time: {:?}", start.elapsed());
    // let start = std::time::Instant::now();
    // day1::part_1_task_group(&world);
    // println!("time: {:?}", start.elapsed());
    // let start = std::time::Instant::now();
    // day1::part_2(&world);
    // println!("time: {:?}", start.elapsed());
    // let start = std::time::Instant::now();
    // day1::part_2_task_group(&world);
    // println!("time: {:?}", start.elapsed());

    // let start = std::time::Instant::now();
    // day2::part_1(&world);
    // println!("time: {:?}", start.elapsed());
    // let start = std::time::Instant::now();
    // day2::part_1_task_group(&world);
    // println!("time: {:?}", start.elapsed());
    // let start = std::time::Instant::now();
    // day2::part_2(&world);
    // println!("time: {:?}", start.elapsed());
    // let start = std::time::Instant::now();
    // day2::part_2_task_group(&world);
    // println!("time: {:?}", start.elapsed());

    // let start = std::time::Instant::now();
    // day3::part_1(&world);
    // println!("time: {:?}", start.elapsed());
    // let start = std::time::Instant::now();
    // day3::part_2(&world);
    // println!("time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    day4::part_1(&world);
    println!("time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    day4::part_2(&world);
    println!("time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    day4::part_2_v2(&world);
    println!("time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    day4::part_2_serial(&world);
    println!("time: {:?}", start.elapsed());
}
