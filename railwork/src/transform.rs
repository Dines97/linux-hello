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
    pub fn from_transform<T>(mut run: T, receiver: Receiver<T::Input>, sender: Sender<T::Output>) -> Self
    where
        T: Transform + 'static,
        T::Input: Send,
        T::Output: Send,
    {
        Self::spawn_thread(move |input| run.run(input), receiver, sender)
    }

    pub fn from_closure<F, Input, Output>(run: F, receiver: Receiver<Input>, sender: Sender<Output>) -> Self
    where
        F: FnMut(Input) -> Output + Send + 'static,
        Input: Send + 'static,
        Output: Send + 'static,
    {
        Self::spawn_thread(run, receiver, sender)
    }

    fn spawn_thread<F, Input, Output>(mut run: F, receiver: Receiver<Input>, sender: Sender<Output>) -> Self
    where
        F: FnMut(Input) -> Output + Send + 'static,
        Input: Send + 'static,
        Output: Send + 'static,
    {
        let continue_flag = Arc::new(Mutex::new(true));
        let continue_flag_clone = Arc::clone(&continue_flag);

        let handle = Some(thread::spawn(move || {
            while *continue_flag_clone.lock().unwrap() {
                if let Ok(input) = receiver.recv() {
                    let output = run(input);
                    let _ = sender.send(output);
                }
            }
        }));

        Self { handle, continue_flag }
    }
}
