use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

use uuid::Uuid;

fn find_me_a_collision(mutex: Arc<Mutex<Vec<u64>>>) {
    let mut partitions = HashSet::new();
    let mut i = 0;

    loop {
        // if i == 1000000000 {
        //     let mut guard = mutex.lock().unwrap();
        //     guard.push(i);
        //     break;
        // }

        let (_, partition_id) = Uuid::new_v4().as_u64_pair();

        if partitions.contains(&partition_id) {
            let mut guard = mutex.lock().unwrap();
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
        for _ in 0..10 {
            s.spawn(|| {
                let mutex = mutex.clone();
                find_me_a_collision(mutex);
            });
        }
    });

    println!("{:#?}", mutex.lock().unwrap());
}
