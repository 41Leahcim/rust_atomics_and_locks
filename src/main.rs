use std::thread;

fn main() {
    let numbers = [1, 2, 3];

    thread::spawn(move || {
        for number in numbers {
            println!("{number}");
        }
    })
    .join()
    .unwrap();
}
