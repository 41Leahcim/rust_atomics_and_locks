use channel::Channel;
use std::thread;

mod channel;

const MESSAGE: &str = "Hello, world!";

fn main() {
    let mut channel = Channel::new();
    thread::scope(|scope| {
        let (sender, receiver) = channel.split();
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
