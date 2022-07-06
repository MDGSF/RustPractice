use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;

#[derive(Debug)]
pub struct Item<T> {
    pub key: String,
    pub val: T,
}

pub struct ThreadSafeVec<T> {
    pub vector: RwLock<Vec<Arc<Mutex<Item<T>>>>>,
}

impl<T> ThreadSafeVec<T> {
    pub fn new() -> Self {
        ThreadSafeVec {
            vector: RwLock::new(vec![]),
        }
    }

    pub fn add(&self, e: Item<T>) {
        let mut vector = self.vector.write().unwrap();
        (*vector).push(Arc::new(Mutex::new(e)));
    }

    pub fn find(&self, key: &str) -> Option<Arc<Mutex<Item<T>>>> {
        let vector = &*self.vector.read().unwrap();
        let mut index: i32 = -1;
        for i in 0..vector.len() {
            if key == vector[i].lock().unwrap().key {
                index = i as i32;
                break;
            }
        }

        if index >= 0 {
            Some(vector[index as usize].clone())
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.vector.read().unwrap().len()
    }
}

#[derive(Debug)]
pub struct Val {
    name: String,
    id: i32,
}

impl Val {
    pub fn run(&self) {
        for i in 0..self.id {
            println!("name = {}, i = {}", self.name, i);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    pub fn modify(&mut self) {
        self.id += 1;
    }
}

fn main() {
    println!("Hello, world!");

    let vector = ThreadSafeVec::<Val>::new();

    {
        let v1 = Val {
            name: String::from("val1"),
            id: 5,
        };

        let v2 = Val {
            name: String::from("val2"),
            id: 5,
        };

        let e1 = Item::<Val> {
            key: String::from("val1"),
            val: v1,
        };

        let e2 = Item::<Val> {
            key: String::from("val2"),
            val: v2,
        };

        vector.add(e1);
        vector.add(e2);
    }

    println!("vector.len = {}", vector.len());

    {
        let item1 = vector.find("val1");
        let item2 = vector.find("val2");
        println!("item1 = {:?}", item1);
        println!("item2 = {:?}", item2);

        let handle = thread::spawn(move || {
            item1.as_ref().unwrap().lock().unwrap().val.run();
            item1.as_ref().unwrap().lock().unwrap().val.modify();
            item1.as_ref().unwrap().lock().unwrap().val.run();
        });
        item2.unwrap().lock().unwrap().val.run();
        handle.join();
    }
}
