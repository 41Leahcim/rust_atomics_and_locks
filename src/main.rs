use channel::Channel;
use std::{sync::Arc, thread};

mod channel;

const MESSAGE: &str = "Hello, world!";

fn main() {
    thread::scope(|scope| {
        let channel = Arc::new(Channel::new());
        let current_thread = thread::current();
        scope.spawn({
            let channel = channel.clone();
            move || {
                channel.send(MESSAGE);
                current_thread.unpark();
            }
        });
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!(channel.receive(), MESSAGE);
    })
}
