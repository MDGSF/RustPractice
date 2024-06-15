use std::sync::Arc;

struct SafeVec<T> {
    vec: Vec<T>,
}

unsafe impl<T> Sync for SafeVec<T> where T: Sync + Send {}
unsafe impl<T> Send for SafeVec<T> where T: Sync + Send {}

impl<T> SafeVec<T> {
    pub fn new(size: usize) -> Self {
        Self {
            vec: Vec::with_capacity(size),
        }
    }

    pub fn index(&self, index: usize) -> *mut T {
        unsafe {
            let ptr = self.vec.as_ptr();
            let ptr = ptr.add(index);
            ptr as *mut T
        }
    }
}

fn main() {
    std::thread::scope(|s| {
        let v: Arc<SafeVec<i32>> = Arc::new(SafeVec::new(10));

        for i in 0..10 {
            let v = v.clone();
            s.spawn(move || {
                let p = v.index(i);
                for j in 1..10000 {
                    unsafe {
                        *p += j;
                    }
                }
            });
        }
    });
}
