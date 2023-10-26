use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1_000);

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", numbers.len());
        });

        scope.spawn(|| {
            // Only iterate over &numbers or numbers.iter().
            // Mutation and ownership are not allowed, because another thread is using numbers.
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}
