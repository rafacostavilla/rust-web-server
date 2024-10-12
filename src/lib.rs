pub struct ThreadPool{}

impl ThreadPool{
    pub fn new(number_of_threads:usize) -> ThreadPool{
        ThreadPool{}
    }
    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static
    {

    }
}