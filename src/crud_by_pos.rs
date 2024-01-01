pub trait CrudByPos<T> {
    fn add(&self, text: &str) -> T;
    fn update(&self, position: usize, text: &str) -> T;
    fn remove(&self, position: usize) -> T;
    fn all(&self) -> Vec<T>;
    fn get(&self, position: usize) -> T;
}