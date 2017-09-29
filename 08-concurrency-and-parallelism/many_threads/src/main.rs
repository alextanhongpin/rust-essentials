extern crate num_cpus;
extern crate threadpool;

use std::{thread, time};
use threadpool::ThreadPool;

fn main() {
    let ncpus = num_cpus::get();
    println!("The number of cpus in this machine is {}", ncpus);

    let pool = ThreadPool::new(ncpus);
    for i in 0..ncpus {
        pool.execute(move || {
            println!("this is thread number {}", i);
        });
    }
    let ten_millis = time::Duration::from_millis(50);
    thread::sleep(ten_millis);
}
