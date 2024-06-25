use channel::Channel;
use std::{hint::spin_loop, sync::Arc, thread};

mod channel;

fn main() {
    thread::scope(|scope| {
        let channel = Arc::new(Channel::new());
        scope.spawn({
            let channel = channel.clone();
            move || {
                channel.send(1);
            }
        });
        while !channel.is_ready() {
            spin_loop();
        }
        println!("{}", channel.receive());
    })
}
