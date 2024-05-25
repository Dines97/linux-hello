use crossbeam_channel::Receiver;

use crate::action::Action;

pub struct SyncActionBlock {}

impl SyncActionBlock {
    pub fn new<T>(mut run: T, receiver: Receiver<T::Input>) -> Self
    where
        T: Action + 'static,
        T::Input: Send,
    {
        let input = receiver.recv().unwrap();
        run.run(input);

        Self {  }
    }
}
