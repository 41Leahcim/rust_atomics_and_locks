use std::{cell::Cell, marker::PhantomData};

// Structs with Cells aren't Sync, but they are send
struct _NotSyncIsSend {
    _handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}

// Structs with pointers are neither Sync nor Send
struct OnlySyncAndSendIfImplemented {
    _p: *mut i32,
}

// Send and Sync aren't safe to implement as it can't be checked by the compiler.
// Variables that aren't Send can't be move across threads
unsafe impl Send for OnlySyncAndSendIfImplemented {}
unsafe impl Sync for OnlySyncAndSendIfImplemented {}

fn main() {}
