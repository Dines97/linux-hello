use crate::{
    cycle_controller::CycleController,
    link::{Link, LinkError, LinkRecvError},
    nodes::cycle::CycledNode,
};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use std::time::Duration;

pub struct TransformNodeBuilder<F, Input, Output> {
    func: Box<F>,
    cycle_controller: bool,
    name: String,

    input_link: Option<Link<Input>>,
    output_link: Option<Link<Output>>,
}

impl<F, Input, Output> TransformNodeBuilder<F, Input, Output>
where
    F: FnMut(Input) -> Output + Send + 'static,
    Input: Send + 'static,
    Output: Send + 'static,
{
    pub fn new(func: F) -> Self {
        Self {
            func: Box::new(func),
            cycle_controller: false,
            name: String::default(),
            input_link: None,
            output_link: None,
        }
    }

    pub fn cycle_controller(mut self, enable: bool) -> Self {
        self.cycle_controller = enable;
        self
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn input_link(mut self, link: &Link<Input>) -> Self {
        self.input_link = Some(link.clone());
        self
    }

    pub fn output_link(mut self, link: &Link<Output>) -> Self {
        self.output_link = Some(link.clone());
        self
    }

    pub fn build(mut self) -> TransformNode {
        let mut cycle_controller = self.cycle_controller.then(CycleController::default);
        let input_link = self.input_link.unwrap_or_default();
        let output_link = self.output_link.unwrap_or_default();

        let inner = CycledNode::new(move |x| match input_link.try_recv() {
            Ok(input) => {
                let output = (self.func)(input);
                if let Some(x) = cycle_controller.as_mut() {
                    log::trace!("{}, {}", self.name, x);
                    x.update();
                }
                if let Err(e) = output_link.send(output) {
                    log::error!("Unable to send: {}", e);
                    *x.lock().unwrap() = false;
                };
            }
            Err(e) => {
                if let LinkError::LinkRecvError(LinkRecvError::TryRecvError(e)) = e {
                    match e {
                        TryRecvError::Empty => std::thread::sleep(Duration::from_millis(10)),
                        TryRecvError::Disconnected => {
                            log::error!("Unable to receive: {}", e);
                            *x.lock().unwrap() = false;
                        }
                    }
                }
            }
        });

        TransformNode { inner }
    }
}

pub struct TransformNode {
    inner: CycledNode,
}

impl TransformNode {
    pub fn new<F, Input, Output>(
        mut func: F,
        receiver: Receiver<Input>,
        sender: Sender<Output>,
        enable_cycle_controller: bool,
        name: String,
    ) -> Self
    where
        F: FnMut(Input) -> Output + Send + 'static,
        Input: Send + 'static,
        Output: Send + 'static,
    {
        let mut cycle_controller = enable_cycle_controller.then(CycleController::default);

        let inner = CycledNode::new(move |x| {
            log::trace!("{},{}", name, receiver.len());
            match receiver.try_recv() {
                Ok(input) => {
                    let output = func(input);
                    if let Some(x) = cycle_controller.as_mut() {
                        log::trace!("{},{}", name, x);
                        x.update();
                    }
                    if let Err(e) = sender.send(output) {
                        log::error!("Unable to send: {}", e);
                        *x.lock().unwrap() = false;
                    };
                }
                Err(e) => match e {
                    TryRecvError::Empty => std::thread::sleep(Duration::from_millis(10)),
                    TryRecvError::Disconnected => {
                        log::error!("Unable to receive: {}", e);
                        *x.lock().unwrap() = false;
                    }
                },
            }
        });

        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A {
        acc: i32,
    }

    impl A {
        fn run(&mut self, x: i32) -> i32 {
            self.acc += x;
            self.acc
        }
    }

    #[test]
    fn it_works() {
        let mut a = A { acc: 2 };

        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();

        let _ = tx1.send(2);
        let _ = tx1.send(4);
        let _ = tx1.send(8);

        let _block = TransformNode::new(move |x| a.run(x), rx1, tx2, false, String::new());

        assert_eq!(rx2.recv().unwrap(), 4);
        assert_eq!(rx2.recv().unwrap(), 8);
        assert_eq!(rx2.recv().unwrap(), 16);
    }
}
