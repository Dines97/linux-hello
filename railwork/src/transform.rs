use crossbeam_channel::{Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub trait Transform: Send {
    type Input;
    type Output;
    fn run(&mut self, input: Self::Input) -> Self::Output;
}

pub struct TransformBlock {
    handle: Option<thread::JoinHandle<()>>,
    continue_flag: Arc<Mutex<bool>>,
}

impl TransformBlock {
    pub fn new<T>(mut run: T, receiver: Receiver<T::Input>, sender: Sender<T::Output>) -> Self
    where
        T: Transform + 'static,
        T::Input: Send,
        T::Output: Send,
    {
        let continue_flag = Arc::new(Mutex::new(true));
        let continue_flag_clone = Arc::clone(&continue_flag);

        let handle = Some(thread::spawn(move || {
            while *continue_flag_clone.lock().unwrap() {
                log::trace!("Len: {}", receiver.len());
                let input = receiver.recv().unwrap();
                let output = run.run(input);
                let _ = sender.send(output);
            }
        }));

        Self { handle, continue_flag }
    }
}
