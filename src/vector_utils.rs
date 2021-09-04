pub mod utils {
    pub fn add<T>(mut v1: Vec<T>, mut v2: Vec<T>) -> Vec<T> {
        v1.append(&mut v2); v1
    }
    pub fn init<T>(mut v: Vec<T>) -> Vec<T> {
        v.pop(); v
    }
    pub fn tail<T>(mut v: Vec<T>) -> Vec<T> {
        v.remove(0); v
    }
    pub fn last<T>(mut v: Vec<T>) -> T {
        v.pop().unwrap()
    }
    pub fn head<T: std::clone::Clone>(v: Vec<T>) -> T {
        last(v[0..1].to_vec())
    }
}