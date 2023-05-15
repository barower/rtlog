use std::{
    time::Duration,
    thread
};
use rtlog::{Rtl, RtlSender, RtlReceiver};
use assert_no_alloc::*;

#[cfg(debug_assertions)]
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

fn log_thread(receiver: RtlReceiver) {
    loop {
        let log = receiver.recv();
        println!("{}: {}", log.counter, log.log);
        thread::sleep(Duration::from_millis(10));
    }
}

fn rt_thread(sender: RtlSender) {
    assert_no_alloc(|| {
        loop {
            thread::sleep(Duration::from_millis(1));
            sender.send("rt_thread stuff".to_string());
        }
    })
}

fn main() {
    let (tx, rx) = Rtl::channel(10);

    let rthread = thread::spawn(move || { rt_thread(tx); });
    let logthread = thread::spawn(move || { log_thread(rx); });

    rthread.join().unwrap();
    logthread.join().unwrap();
}
