use channel::Channel;
use std::{sync::Arc, thread};

mod channel;

fn main() {
    thread::scope(|scope| {
        let channel = Arc::new(Channel::new());
        scope.spawn({
            let channel = channel.clone();
            move || {
                for i in 0..10 {
                    channel.send(i);
                }
            }
        });
        for _ in 0..10 {
            println!("{}", channel.receive());
        }
    })
}
