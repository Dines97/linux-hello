use std::{
    marker::Send,
    thread::{self, JoinHandle},
};

pub(crate) struct ThreadNode {
    handle: Option<JoinHandle<()>>,
}

impl ThreadNode {
    pub(crate) fn new<F>(func: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        let handle = Some(thread::spawn(func));

        Self { handle }
    }
}

impl Drop for ThreadNode {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Unable to join thread for whatever reason");
        }
    }
}
