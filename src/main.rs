use channel::Channel;
use std::thread;

mod channel;

const MESSAGE: &str = "Hello, world!";

fn main() {
    let mut channel = Channel::new();
    thread::scope(|scope| {
        let (sender, receiver) = channel.split();
        scope.spawn(move || {
            sender.send(MESSAGE);
        });
        assert_eq!(receiver.receive(), MESSAGE);
    })
}
