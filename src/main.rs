use channel::channel;
use std::thread;

mod channel;

const MESSAGE: &str = "Hello, world!";

fn main() {
    thread::scope(|scope| {
        let (sender, receiver) = channel();
        let current_thread = thread::current();
        scope.spawn(move || {
            sender.send(MESSAGE);
            current_thread.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), MESSAGE);
    })
}
