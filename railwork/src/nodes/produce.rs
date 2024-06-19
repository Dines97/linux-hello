use crate::{cycle_controller::CycleController, link::Link, nodes::cycle::CycledNode};
use crossbeam_channel::Sender;

pub struct ProduceNodeBuilder<F, Output>
where
    F: FnMut() -> Output + Send + 'static,
    Output: Send + 'static,
{
    func: Box<F>,
    cycle_controller: bool,
    output_link: Option<Link<Output>>,
}

impl<F, Output> ProduceNodeBuilder<F, Output>
where
    F: FnMut() -> Output + Send + 'static,
    Output: Send + 'static,
{
    pub fn new(func: F) -> Self {
        Self {
            func: Box::new(func),
            cycle_controller: false,
            output_link: None,
        }
    }

    pub fn cycle_controller(mut self, enable: bool) -> Self {
        self.cycle_controller = enable;
        self
    }

    pub fn output_link(mut self, link: &Link<Output>) -> Self {
        self.output_link = Some(link.clone());
        self
    }

    pub fn build(mut self) -> ProduceNode {
        let mut cycle_controller = self.cycle_controller.then(CycleController::default);
        let link = self.output_link.unwrap_or_default();

        let inner = CycledNode::new(move |x| {
            let output = (self.func)();
            if let Some(x) = cycle_controller.as_mut() {
                log::trace!("{}", x);
                x.update();
            }
            if let Err(e) = link.send(output) {
                log::error!("Unable to send: {}", e);
                *x.lock().unwrap() = false;
            };
        });

        ProduceNode { inner }
    }
}

pub struct ProduceNode {
    inner: CycledNode,
}

impl ProduceNode {
    pub fn new<F, Output>(mut func: F, sender: Sender<Output>, enable_cycle_controller: bool) -> Self
    where
        F: FnMut() -> Output + Send + 'static,
        Output: Send + 'static,
    {
        let mut cycle_controller = enable_cycle_controller.then(CycleController::default);

        let inner = CycledNode::new(move |x| {
            let output = func();
            if let Some(x) = cycle_controller.as_mut() {
                log::trace!("{}", x);
                x.update();
            }
            if let Err(e) = sender.send(output) {
                log::error!("Unable to send: {}", e);
                *x.lock().unwrap() = false;
            };
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
        fn run(&mut self) -> i32 {
            self.acc = (2 * self.acc) % 2048;
            self.acc
        }
    }

    #[test]
    fn it_works() {
        let mut a = A { acc: 2 };

        let (tx, rx) = crossbeam_channel::unbounded();

        let _block = ProduceNode::new(move || a.run(), tx, false);
        // rx.recv().unwrap();

        assert_eq!(rx.recv().unwrap(), 4);
        assert_eq!(rx.recv().unwrap(), 8);
        assert_eq!(rx.recv().unwrap(), 16);
    }
}
