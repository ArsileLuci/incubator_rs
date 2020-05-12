#[macro_use(lazy_static)]
extern crate lazy_static;

use core::cell::Ref;
use std::cell::RefCell;
use std::sync::MutexGuard;
use std::sync::{Arc, Mutex, Weak};
use std::{marker::PhantomData, thread};

#[derive(Debug)]
struct DLLNode<T> {
    pub next: Option<Arc<RefCell<DLLNode<T>>>>,
    pub prev: Weak<RefCell<DLLNode<T>>>,
    pub field: T,
}

unsafe impl<T> Send for DLLNode<T> where T: Send + Sync {}
unsafe impl<T> Sync for DLLNode<T> where T: Sync + Send {}

impl<T> DLLNode<T> {
    pub fn set_prev(&mut self, prev: Weak<RefCell<DLLNode<T>>>) {
        self.prev = prev;
    }
    pub fn drop_prev(&mut self) {
        self.prev = Weak::new();
    }
    pub fn set_next(&mut self, next: Arc<RefCell<DLLNode<T>>>) {
        self.next = Some(next);
    }
    pub fn drop_next(&mut self) {
        self.next = None;
    }
}
#[derive(Debug)]
struct DLLStore<T>
where
    T: Send + Sync,
{
    start: Option<Arc<RefCell<DLLNode<T>>>>,
    end: Option<Arc<RefCell<DLLNode<T>>>>,
    len: usize,
}

unsafe impl<T> Send for DLLStore<T> where T: Send + Sync {}
unsafe impl<T> Sync for DLLStore<T> where T: Send + Sync {}

impl<T> DLLStore<T>
where
    T: Send + Sync,
{
    pub fn new() -> Self {
        DLLStore {
            start: None,
            end: None,
            len: 0,
        }
    }
    pub fn push_back(&mut self, val: T) {
        match self.len {
            0 => {
                let arc = Arc::new(RefCell::new(DLLNode {
                    prev: Weak::new(),
                    next: None,
                    field: val,
                }));
                self.start = Some(arc);
            }
            1 => {
                let arc = Arc::new(RefCell::new(DLLNode {
                    next: None,
                    prev: Arc::downgrade(self.start.as_ref().unwrap()),
                    field: val,
                }));
                self.end = Some(arc);
                self.start.as_ref().unwrap().borrow_mut().next =
                    Some(self.end.as_ref().unwrap().clone());
            }
            _ => {
                let arc = Arc::new(RefCell::new(DLLNode {
                    next: None,
                    prev: Arc::downgrade(self.end.as_ref().unwrap()),
                    field: val,
                }));
                let end_arc = self.end.as_ref().unwrap();
                end_arc.borrow_mut().next = Some(arc.clone());
                self.end = Some(arc);
            }
        }
        self.len += 1
    }
    pub fn push_front(&mut self, val: T) {
        match self.len {
            0 => {
                let arc = Arc::new(RefCell::new(DLLNode {
                    prev: Weak::new(),
                    next: None,
                    field: val,
                }));
                self.start = Some(arc);
            }
            1 => {
                self.end = Some(self.start.as_ref().unwrap().clone());
                let arc = Arc::new(RefCell::new(DLLNode {
                    prev: Weak::new(),
                    next: Some(self.start.as_ref().unwrap().clone()),
                    field: val,
                }));
                self.start = Some(arc);
                self.end.as_ref().unwrap().borrow_mut().prev =
                    Arc::downgrade(self.start.as_ref().unwrap());
            }
            _ => {
                let arc = Arc::new(RefCell::new(DLLNode {
                    prev: Weak::new(),
                    next: Some(self.start.as_ref().unwrap().clone()),
                    field: val,
                }));
                let start_arc = self.start.as_ref().unwrap();
                start_arc.borrow_mut().prev = Arc::downgrade(&arc);
                self.start = Some(arc);
            }
        }
        self.len += 1
    }

    pub fn peek_front(&self) -> T
    where
        T: Copy,
    {
        self.start.as_ref().unwrap().borrow().field
    }

    pub fn peek_back(&self) -> T
    where
        T: Copy,
    {
        self.end.as_ref().unwrap().borrow().field
    }

    pub fn drop_start(&mut self) -> Option<T>
    where
        T: std::fmt::Debug,
    {
        if self.start.is_none() {
            return None;
        }

        let start = self.start.as_ref().clone().unwrap().clone();
        let new_start: Option<Arc<RefCell<DLLNode<T>>>>;
        match start.borrow().next.as_ref() {
            None => new_start = None,
            Some(val) => new_start = Some(val.clone()),
        }
        self.start = new_start;
        if self.len == 1 {
            self.end = None
        }
        let result = Arc::try_unwrap(start);
        let node = result.unwrap().into_inner();
        self.len -= 1;
        return Some(node.field);
    }

    pub fn search(&mut self, val: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        let mut counter = 0;
        match self.start.as_ref() {
            None => None,
            Some(cur) => {
                let mut next = cur.clone();
                loop {
                    if &next.borrow().field == val {
                        return Some(counter);
                    }
                    if next.borrow().next.is_none() {
                        break;
                    } else {
                        let swap = next.borrow().next.as_ref().unwrap().clone();
                        next = swap;
                    }

                    counter += 1;
                }
                None
            }
        }
    }

    pub fn replace_at(&mut self, val: T, index: usize) -> Option<T> {
        if self.len <= index || index < 0 {
            return None;
        }
        let mut ptr = self.start.as_ref().unwrap().clone();
        let mut curent: usize = 0;
        while curent < index {
            let newpos = ptr.borrow().next.as_ref().unwrap().clone();
            ptr = newpos;
            curent += 1;
        }
        let result = std::mem::replace(&mut ptr.borrow_mut().field, val);
        return Some(result);
    }

    pub fn drop_end(&mut self) -> Option<T>
    where
        T: std::fmt::Debug,
    {
        if self.start.is_none() {
            return None;
        }
        if (self.end.is_none()) {
            let node = self.start.as_ref().unwrap().clone();
            self.start = None;
            self.len -= 1;
            let result = Arc::try_unwrap(node);
            let node = result.unwrap().into_inner();
            return Some(node.field);
        }

        let end = self.end.as_ref().clone().unwrap().clone();
        if self.len > 1 {
            let new_end: Option<Arc<RefCell<DLLNode<T>>>>;
            match end.borrow().prev.upgrade() {
                None => new_end = None,
                Some(val) => new_end = Some(val.clone()),
            }
            self.end = new_end;
        }
        if self.len == 1 {
            self.end = None
        }
        let result = Arc::try_unwrap(end);
        let node = result.unwrap().into_inner();
        self.len -= 1;
        return Some(node.field);
    }
}
struct DLLIter<T>
where
    T: Sync + Send + std::fmt::Debug,
{
    dll: DoubleLinkedList<T>,
}
impl<T> Iterator for DLLIter<T>
where
    T: Sync + Send + std::fmt::Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.dll.drop_front()
    }
}

impl<T> IntoIterator for DoubleLinkedList<T>
where
    T: Send + Sync + std::fmt::Debug,
{
    type IntoIter = DLLIter<T>;
    type Item = T;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        DLLIter { dll: self }
    }
}

struct DoubleLinkedList<T>
where
    T: Send + Sync,
{
    inner_storage: Arc<Mutex<DLLStore<T>>>,
}

impl<T> DoubleLinkedList<T>
where
    T: Send + Sync + std::fmt::Debug,
{
    pub fn new() -> Self {
        DoubleLinkedList {
            inner_storage: Arc::new(Mutex::new(DLLStore::new())),
        }
    }

    pub fn push_back(&self, val: T) {
        let mut lock = self.inner_storage.lock().unwrap();
        lock.push_back(val);
    }

    pub fn push_front(&self, val: T) {
        let mut lock = self.inner_storage.lock().unwrap();
        lock.push_front(val);
    }

    pub fn drop_front(&self) -> Option<T>
    where
        T: std::fmt::Debug,
    {
        let mut lock = self.inner_storage.lock().unwrap();
        lock.drop_start()
    }
    pub fn search(&self, val: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.inner_storage.lock().unwrap().search(val)
    }

    pub fn replace_at(&self, val: T, index: usize) -> Option<T> {
        self.inner_storage.lock().unwrap().replace_at(val, index)
    }
}

fn main() {
    lazy_static! {
        static ref DLL: DoubleLinkedList<u8> = DoubleLinkedList::<u8>::new();
    }
    let t1 = thread::spawn(|| {
        for i in 0..255 {
            &DLL.push_back(i);
        }
    });
    let t2 = thread::spawn(|| {
        for i in 0..255 {
            &DLL.push_front(i);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
