use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use uuid::Uuid;

fn find_me_a_collision(mut guard: MutexGuard<Vec<u64>>) {
    let mut partitions = HashSet::new();
    let mut i = 0;

    loop {
        let (_, partition_id) = Uuid::new_v4().as_u64_pair();

        if partitions.contains(&partition_id) {
            guard.push(i);
            break;
        }

        partitions.insert(partition_id);
        i = i + 1;
    }
}

fn main() {
    let mutex: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for _ in 0..2 {
            s.spawn(|| {
                find_me_a_collision(mutex.lock().unwrap());
            });
        }
    });

    let guard = mutex.lock().unwrap();
    println!("{:#?}", guard);
}
