use std::thread;
use std::{collections::VecDeque, sync::Condvar};
use std::sync::Mutex;
fn main() {
    let queue : Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(||{
            loop {
                let mut q_guard = queue.lock().unwrap();
                let item =  loop {
                    if let Some(item) = q_guard.pop_front() {
                        break item;
                    } else {
                        q_guard = not_empty.wait(q_guard).unwrap();
                    }
                };
                drop(q_guard);
                dbg!(item);
            }
        });
    });
}
