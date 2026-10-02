use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type target = T;

    fn deref(&self) -> Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = MySmartPointer::new(Box::new("Let's Get Rusty.".to_owned()));
    // In this case,
    // &MySmartPointer -> &Box -> &String -> &str
    print(&s); // This works because feature in rust, implicit `deref coercion`:  allows the rust compiler to coercise the refernce of one type to another type. 
}

fn print(s: &str) {
    println!("{s}");
}
