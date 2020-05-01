use std::fmt;
use std::pin::Pin;
use std::fmt::Debug;
use std::rc::*;

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

//Box<T>, Rc<T>, Vec<T>, String, &[u8], T
#[derive(Debug)]
struct MyMutable {
    pub field: i32
}
impl Unpin for MyMutable {
    // add code here
}


impl<T> SayHi for Box<T>
    where T: Debug {}

impl<T> SayHi for Rc<T>
    where T: Debug {}

impl<T> SayHi for Vec<T>
    where T: Debug {}

impl SayHi for String {}

impl SayHi for &[u8] {}

impl<T> MutMeSomehow for Box<T>
    where T: Unpin + Default {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let x = self.get_mut().as_mut();
        (*x) = T::default();
    }
}
 

impl<T> MutMeSomehow for Rc<T>
    where T: Unpin + Default {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let mut rc = self.get_mut();
        let val = std::rc::Rc::<T>::get_mut(&mut rc)
            .unwrap();
        (*val) = T::default();
    }
}

impl<T> MutMeSomehow for Vec<T>
    where T: Unpin {
    fn mut_me_somehow(self: Pin<&mut Self>){
        let v = self.get_mut();
        if v.len()>0 {
           v.pop();
        }
    }
}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>){
        let text = self.get_mut();
        text.push_str("aaaaa");
    }
}
//TODO Uncomment

impl SayHi for MyMutable {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self);
    }
}

impl MutMeSomehow for MyMutable {
   fn mut_me_somehow(self: Pin<&mut Self>) {
        let my_mut = self.get_mut();
        my_mut.field = 100;
    }
 }

fn main() {
    //Box
    let mut bx = std::boxed::Box::<i32>::new(12);
    MutMeSomehow::mut_me_somehow(Pin::new(&mut bx));
    SayHi::say_hi(Pin::new(&bx));

    //rc
    let mut rc = Rc::<i32>::new(10);
    MutMeSomehow::mut_me_somehow(Pin::new(&mut rc));
    SayHi::say_hi(Pin::new(&rc));

    //Vec
    let mut v = vec![10,20,30];
    MutMeSomehow::mut_me_somehow(Pin::new(&mut v));
    SayHi::say_hi(Pin::new(&v));

    //String
    let mut s = String::from("my string");
    MutMeSomehow::mut_me_somehow(Pin::new(&mut s));
    SayHi::say_hi(Pin::new(&s));

    //MyMutable
    let mut mm  = MyMutable{field:12};
    MutMeSomehow::mut_me_somehow(Pin::new(&mut mm));
    SayHi::say_hi(Pin::new(&mm))
}
