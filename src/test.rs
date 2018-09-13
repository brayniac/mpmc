use super::*;
use alloc::String;
use spawn::KernelTaskBuilder;

// static mut TEST_QUEUE: Queue::<u64> = Queue::with_capacity(200);

fn test_enqueue1(q : Queue<u64>) {

    for _ in 0..100 {
        let _ = q.push(1);
    }
    
}

fn test_enqueue2(q : Queue<u64>) {
    for _ in 0..100 {
        let _ = q.push(2);
    }
}

fn test_dequeue1(q : Queue<u64>) {

    for _ in 0..100 {
        debug!("thread 1: {}",q.pop().unwrap());
    }
    
}

fn test_dequeue2(q : Queue<u64>) {
    for _ in 0..100 {
        debug!("thread 2: {}",q.pop().unwrap());
    }
}

pub fn test_multithread_mpmc () {
    let q = Queue::<u64>::with_capacity(2000);

    let task1 = KernelTaskBuilder::new(test_enqueue1, q.clone())
            .name(String::from("test_enqueue1"))
            .spawn();

    let task2 = KernelTaskBuilder::new(test_enqueue2, q.clone())
            .name(String::from("test_enqueue2"))
            .spawn();
    
    let _ = task1.unwrap().join();
	let _ = task2.unwrap().join();

    // let mut count = 0;

    // loop {
    //     let a = q.pop();

    //     if a == None {
    //         debug!("{}", count);
    //         break;
    //     }
    //     else {
    //         count += 1;
    //         debug!("{}", a.unwrap());
    //     }
    // }

    let task3 = KernelTaskBuilder::new(test_dequeue1, q.clone())
            .name(String::from("test_dequeue1"))
            .spawn();

    let task4 = KernelTaskBuilder::new(test_dequeue2, q.clone())
            .name(String::from("test_dequeue2"))
            .spawn();
    
    let _ = task3.unwrap().join();
	let _ = task4.unwrap().join();
}

