use std::sync::{
    Arc,
    mpsc,
    atomic::AtomicU64
};

pub struct Rtl;

pub struct RtlMessage{
    pub counter: u64,
    pub log: String
}

pub struct RtlSender{
    sender: mpsc::SyncSender<RtlMessage>,
    counter: Arc<AtomicU64>
}

pub struct RtlReceiver{
    receiver: mpsc::Receiver<RtlMessage>,
}

impl Rtl {
    pub fn channel(buffer_size: usize) -> (RtlSender, RtlReceiver) {
        let counter = Arc::new(AtomicU64::new(0));
        let (sender, receiver) = mpsc::sync_channel(buffer_size);
        (RtlSender{sender, counter}, RtlReceiver{receiver})
    }
}

impl RtlMessage {
    fn new(counter: u64, log: String) -> Self {
        RtlMessage {counter, log}
    }
}

impl RtlSender {
    pub fn send(&self, log: String) {
        let counter = self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let msg = RtlMessage::new(counter, log);
        self.sender.try_send(msg).unwrap_or(())
    }
}

impl RtlReceiver {
    pub fn recv(&self) -> RtlMessage {
        self.receiver.recv().unwrap()
    }
}
