use std::sync::mpsc;

pub struct Rtl;
pub struct RtlSender(mpsc::Sender<String>);
pub struct RtlReceiver(mpsc::Receiver<String>);

impl Rtl {
    pub fn channel() -> (RtlSender, RtlReceiver) {
        let (sender, receiver) = mpsc::channel();
        (RtlSender(sender), RtlReceiver(receiver))
    }
}

impl RtlSender {
    pub fn send(&self, log: String) {
        self.0.send(log).unwrap();
    }
}

impl RtlReceiver {
    pub fn recv(&self) -> String {
        self.0.recv().unwrap()
    }
}
