mod enums;
mod linkedList;
mod node;
mod tests;

use std::{cell::RefCell, rc::Rc};
use std::collections::LinkedList;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}


fn test_i32_doubly_linked_list() -> u128 {
    let mut custom_list = DoublyLinkedList::new();

    let start_custom = SystemTime::now();
    for i in 0..1_000_000 {
        custom_list.push_back(346);
    }
    let end_custom = SystemTime::now();

    let custom_list_time = end_custom.duration_since(start_custom).unwrap().as_millis();
    custom_list_time
}

fn test_f64_doubly_linked_list() -> u128 {
    let mut custom_list = DoublyLinkedList::new();

    let start_custom = SystemTime::now();
    for i in 0..1_000_000 {
        custom_list.push_back(364.3433);
    }
    let end_custom = SystemTime::now();

    let custom_list_time = end_custom.duration_since(start_custom).unwrap().as_millis();
    custom_list_time
}

fn test_complex_doubly_linked_list() -> u128 {
    let mut custom_list = DoublyLinkedList::new();

    let start_custom = SystemTime::now();
    for i in 0..1_000_000 {
        custom_list.push_back(Complex { real: 999.999, imag: 999.999 });
    }
    let end_custom = SystemTime::now();

    // sum of all 1,000,000 elements
    let mut sum: i128 = 0;
    while let Some(node) = custom_list.pop_first() {
        let data1 = node.real;
        let data2 = node.imag;
        sum += data1 as i128 + data2 as i128;
    }

    println!("{}", sum);

    let custom_list_time = end_custom.duration_since(start_custom).unwrap().as_millis();
    custom_list_time
}

struct test_struct{
    id: i32,
    name: String,
    float: f64,
}

fn main() {
    // run it 10 times
    let mut i32_time = 0;
    let mut f64_time = 0;
    let mut complex_time = 0;

    for _ in 0..10 {
        i32_time += test_i32_doubly_linked_list();
        f64_time += test_f64_doubly_linked_list();
        complex_time += test_complex_doubly_linked_list();
    }

    println!("i32: {}ms", i32_time / 10);
    println!("f64: {}ms", f64_time / 10);
    println!("Complex: {}ms", complex_time / 10);
}
