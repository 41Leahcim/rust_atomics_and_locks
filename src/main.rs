use std::thread;

fn main() {
    let threads = (0..2).map(|_| thread::spawn(f)).collect::<Vec<_>>();

    println!("Hello from the main thread.");
    threads
        .into_iter()
        .for_each(|thread| thread.join().unwrap());
}

fn f() {
    let id = thread::current().id();
    println!("Hello from another thread\nThis is my thread id: {id:?}");
}
