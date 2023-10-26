use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1_000);

    let average = thread::spawn(move || {
        let length = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / length
    })
    .join()
    .unwrap();

    println!("Average: {average}");
}
