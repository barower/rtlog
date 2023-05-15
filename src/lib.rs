use std::sync::{
    Arc,
    mpsc,
    atomic::AtomicU64
};

pub struct Rtl;

pub struct RtlSender{
    sender: mpsc::Sender<String>,
    counter: Arc<AtomicU64>
}

pub struct RtlReceiver{
    receiver: mpsc::Receiver<String>,
}

impl Rtl {
    pub fn channel() -> (RtlSender, RtlReceiver) {
        let counter = Arc::new(AtomicU64::new(0));
        let (sender, receiver) = mpsc::channel();
        (RtlSender{sender, counter}, RtlReceiver{receiver})
    }
}

impl RtlSender {
    pub fn send(&self, log: String) {
        let counter = self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let log = format!("{}: {}", counter, log);
        self.sender.send(log).unwrap();
    }
}

impl RtlReceiver {
    pub fn recv(&self) -> String {
        self.receiver.recv().unwrap()
    }
}
