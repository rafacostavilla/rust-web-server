use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool{
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<Worker>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size:usize) -> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));            
        }
        
        ThreadPool{
            sender: Some(sender), 
            workers,
        }
    }
    
    pub fn execute<F>(&self, f:F)
    where
    F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .take()
            .unwrap()
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }

}
struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .unwrap()
                .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                    println!("Worker {id} finished a job; completed.");
                },
                _ =>{
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker{
            id,
            thread: Some(thread),
        }
    }
}