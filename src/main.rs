use std::{
    time::Duration,
    thread
};

use rtlog::{Rtl, RtlSender, RtlReceiver};

fn log_thread(receiver: RtlReceiver) {
    loop {
        let log = receiver.recv();
        println!("{}: {}", log.counter, log.log);
    }
}

fn rt_thread(sender: RtlSender) {
    loop {
        thread::sleep(Duration::from_millis(100));
        sender.send("rt_thread stuff".to_string());
    }
}

fn main() {
    let (tx, rx) = Rtl::channel();

    let rthread = thread::spawn(move || { rt_thread(tx); });
    let logthread = thread::spawn(move || { log_thread(rx); });

    rthread.join().unwrap();
    logthread.join().unwrap();
}
