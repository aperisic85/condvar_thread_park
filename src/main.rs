use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::{collections::VecDeque, sync::Condvar};
fn main() {
    let queue: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            let mut q_guard: std::sync::MutexGuard<'_, VecDeque<u32>> = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q_guard.pop_front() {
                    break item;
                } else {
                    q_guard = not_empty.wait(q_guard).unwrap();
                }
            };
            drop(q_guard);
            dbg!(item);
        });

       

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
