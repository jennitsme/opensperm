use crate::ipc::IpcMessage;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Streamer {
    pub tx: Sender<IpcMessage>,
    pub rx: Receiver<IpcMessage>,
}

impl Streamer {
    pub fn new(buffer: usize) -> Self {
        let (tx, rx) = channel(buffer);
        Self { tx, rx }
    }
}
