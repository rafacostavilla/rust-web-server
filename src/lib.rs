use std::{clone, sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool{
    sender: mpsc::Sender<Job>,
    workers: Vec<Worker>,
}

struct Job;

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
        
        ThreadPool{sender, workers}
    }
    
    pub fn execute<F>(&self, f:F)
    where
    F: FnOnce() + Send + 'static
    {
        
    }
}
struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        Worker{
            id,
            thread: thread::spawn(||{
                receiver;
            }),
        }
    }
}