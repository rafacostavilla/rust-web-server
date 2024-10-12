use std::thread::JoinHandle;

pub struct ThreadPool{
    threads: Vec<JoinHandle<()>>,
}

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
        let threads = Vec::with_capacity(size);

        for _ in 0..size{
            // create some threads and store it in the vector
        }

        ThreadPool{threads}
    }
    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static
    {

    }
}