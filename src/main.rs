use std::sync::{Arc, Mutex};

fn process_item<T>(_item: T) {}

fn do_something() {}

fn main() {
    let list = Arc::new(Mutex::new(vec![3, 2]));

    // Lock the mutex, add 1 element, and unlock the mutex again in 1 statement
    // Lock will return an error if the mutex is poisoned
    list.lock().unwrap().push(1);

    // Pop an element from the list, the mutex will be locked until the end of the if let statement
    if let Some(item) = list.clone().lock().unwrap().pop() {
        process_item(item);
    }

    // Mutex immediately released after the condition
    if list.lock().unwrap().pop() == Some(2) {
        do_something();
    }

    // Mutex will be unlocked after this statement
    let item = list.lock().unwrap().pop();
    if let Some(item) = item {
        process_item(item);
    }
}
