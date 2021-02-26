use std::ops::Add;
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;
use std::cell::RefCell;

fn main() {
    let mut a = 0;
    for i in 0..100{
        a+=1;
    }
}
