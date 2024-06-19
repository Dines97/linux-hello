use crossbeam_channel::{unbounded, Receiver, Sender};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkError<T> {
    #[error("link sender error")]
    LinkSendError(#[from] LinkSendError<T>),

    #[error("link receiver error")]
    LinkRecvError(#[from] LinkRecvError),
}

#[derive(Error, Debug)]
pub enum LinkSendError<T> {
    #[error("crossbeam sender error")]
    SendError(#[from] crossbeam_channel::SendError<T>),
}

#[derive(Error, Debug)]
pub enum LinkRecvError {
    #[error("crossbeam receiver error")]
    RecvError(#[from] crossbeam_channel::RecvError),

    #[error("crossbeam try receiver error")]
    TryRecvError(#[from] crossbeam_channel::TryRecvError),
}

pub struct Link<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Link<T> {
    pub fn send(&self, msg: T) -> Result<(), LinkError<T>> {
        self.sender
            .send(msg)
            .map_err(LinkSendError::SendError)
            .map_err(LinkError::LinkSendError)
    }

    pub fn recv(&self) -> Result<T, LinkError<T>> {
        self.receiver
            .recv()
            .map_err(LinkRecvError::RecvError)
            .map_err(LinkError::LinkRecvError)
    }

    pub fn try_recv(&self) -> Result<T, LinkError<T>> {
        self.receiver
            .try_recv()
            .map_err(LinkRecvError::TryRecvError)
            .map_err(LinkError::LinkRecvError)
    }
}

impl<T> Default for Link<T> {
    fn default() -> Self {
        let (sender, receiver): (Sender<_>, Receiver<_>) = unbounded();

        Self { sender, receiver }
    }
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        }
    }
}
