use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut arr = [83, 12, 13, 35, 91, 71, 75, 58, 26, 38, 2, 23, 10];
    sleep_sort(&mut arr);
    assert_eq!(arr, [2, 10, 12, 13, 23, 26, 35, 38, 58, 71, 75, 83, 91]);
}

fn sleep_sort(arr: &mut [u64]) {
    let mut time_multiplier = 1.0;
    let mut attempts = 0;

    loop {
        let result = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::with_capacity(arr.len());

        // Iterate over the immutable slice
        for &thread_elem in arr.as_ref() {
            // Create a new owner of the Arc to move into the new thread
            let result = result.clone();

            handles.push(thread::spawn(move || {
                let time = (thread_elem as f64 * time_multiplier) as u32;
                thread::sleep(Duration::new(0, time));
                result.lock().unwrap().push(thread_elem);
            }));
        }

        // Make sure our main thread waits until all other threads are finished
        for handle in handles {
            handle.join().unwrap();
        }

        attempts += 1;
        time_multiplier *= 2.0;

        // Now, there should be only one owner of the Arc left: this main
        // thread. Thus we can unwrap it to get the ownership of the vector.
        let result = Arc::try_unwrap(result).unwrap().into_inner().unwrap();

        // If the resulting vector is actually sorted, we can write all
        // elements to the original slice.
        if result.iter().zip(&result[1..]).all(|(&a, &b)| a <= b) {
            for (orig, new) in arr.iter_mut().zip(result) {
                *orig = new;
            }
            break;
        }
    }

    println!("Sorted in {} attempts (multiplier: {})", attempts, time_multiplier);
}
