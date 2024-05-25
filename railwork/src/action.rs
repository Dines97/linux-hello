use crossbeam_channel::Receiver;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub trait Action: Send {
    type Input;
    fn run(&mut self, input: Self::Input);
}

pub struct ActionBlock {
    handle: Option<thread::JoinHandle<()>>,
    continue_flag: Arc<Mutex<bool>>,
}

impl ActionBlock {
    pub fn new<T>(mut run: T, receiver: Receiver<T::Input>) -> Self
    where
        T: Action + 'static,
        T::Input: Send,
    {
        let continue_flag = Arc::new(Mutex::new(true));
        let continue_flag_clone = Arc::clone(&continue_flag);

        let handle = Some(thread::spawn(move || {
            while *continue_flag_clone.lock().unwrap() {
                log::trace!("Len: {}", receiver.len());
                if let Ok(input) = receiver.recv() {
                    run.run(input);
                } else {
                    break;
                }
            }
        }));

        Self { handle, continue_flag }
    }
}
