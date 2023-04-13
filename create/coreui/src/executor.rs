use std::future::Future;

use tokio::task::JoinHandle;

pub struct Executor {
    runtime: tokio::runtime::Runtime,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            runtime: tokio::runtime::Runtime::new().unwrap()
        }
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output> where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }
}


#[cfg(test)]
mod test {
    use tokio::task::futures;

    use super::*;

    #[test]
    fn test() {
        let executor = Executor::new();
        executor.spawn(async {
            println!("hello world")
        });
    }
}