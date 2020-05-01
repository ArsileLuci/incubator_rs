use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct GlobalStack<T>(Arc<Mutex<Vec<T>>>);

impl<T> GlobalStack<T> {
    pub fn new() -> Self {
        GlobalStack { 0: Arc::new(Mutex::new(vec![])) }
    }

    pub fn push(&self, item:T) {
        let mut mutex = self.0.lock().unwrap();
        (*mutex).push(item);
    }
    pub fn pop(&self) -> Option<T> {
        let mut mutex = self.0.lock().unwrap();
        (*mutex).pop()
    }
    pub fn len(&self) -> usize {
        (*self.0.lock().unwrap()).len()
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        GlobalStack{0: self.0.clone() }
    }
}

fn main() {
    let gstack : GlobalStack<i32> = GlobalStack::<i32>::new();
    let t1_gstack = gstack.clone();
    let t2_gstack = gstack.clone();
    let thread1 = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        &t1_gstack.push(100);
        println!("t1: {}",&t1_gstack.len());
        std::thread::sleep(std::time::Duration::from_secs(5));
        &t1_gstack.push(101);
        println!("t1: {}",&t1_gstack.len());
    });
    let thread2 = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        &t2_gstack.push(200);
        println!("t2: {}",&t2_gstack.len());
        std::thread::sleep(std::time::Duration::from_secs(2));
        &t2_gstack.push(201);
        println!("t2: {}",&t2_gstack.len());
        std::thread::sleep(std::time::Duration::from_secs(2));
        &t2_gstack.push(202);
        println!("t2: {}",&t2_gstack.len());
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("{:?}",gstack);
    gstack.pop();
    println!("{:?}",gstack);
    
}
