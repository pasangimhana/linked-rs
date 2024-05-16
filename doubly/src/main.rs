mod enums;
mod linked_list;
mod node;
mod tests;

use std::time::SystemTime;


#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String,
    age: i32,
    email: String,
}

fn test_i32_doubly_linked_list() -> u128 {
    let mut custom_list = linked_list::DoublyLinkedList::new();
    let start_custom = SystemTime::now();
    for i in 0..20_000_000 {
        custom_list.push_back(346);
    }
    let end_custom = SystemTime::now();
    let custom_list_time = end_custom.duration_since(start_custom).unwrap().as_millis();

    return custom_list_time;
}

fn test_f64_doubly_linked_list() -> u128 {
    let mut custom_list = linked_list::DoublyLinkedList::new();
    let start_custom = SystemTime::now();
    for i in 0..20_000_000 {
        custom_list.push_back(364.3433);
    }
    let end_custom = SystemTime::now();
    let custom_list_time = end_custom.duration_since(start_custom).unwrap().as_millis();

    return custom_list_time;
}

fn test_complex_doubly_linked_list() -> u128 {
    let mut custom_list = linked_list::DoublyLinkedList::new();
    let start_custom = SystemTime::now();
    for i in 0..20_000_000 {
        custom_list.push_back(Complex {
            real: 999.999,
            imag: 999.999,
        });
    }
    let end_custom = SystemTime::now();
    let custom_list_time = end_custom.duration_since(start_custom).unwrap().as_millis();

    return custom_list_time;
}

fn test_user_doubly_linked_list() -> u128 {
    let mut custom_list = linked_list::DoublyLinkedList::new();
    let start_custom = SystemTime::now();
    for i in 0..20_000_000 {
        custom_list.push_back(User {
            id: 1,
            name: "John Doe".to_string(),
            age: 30,
            email: "john@gmail.com".to_string(),
        });
    }
    let end_custom = SystemTime::now();
    let custom_list_time = end_custom.duration_since(start_custom).unwrap().as_millis();

    return custom_list_time;
}


fn run_test() -> (u128, u128, u128, u128)
{
    let i32_time = test_i32_doubly_linked_list();
    let f64_time = test_f64_doubly_linked_list();
    let complex_time = test_complex_doubly_linked_list();
    let user_time = test_user_doubly_linked_list();

    return (i32_time, f64_time, complex_time, user_time);
}

fn main() {
        let mut i32_times = Vec::new();
    let mut f64_times = Vec::new();
    let mut complex_times = Vec::new();
    let mut user_times = Vec::new();

    for i in 0..10 {
        let (i32_time, f64_time, complex_time, user_time) = run_test();
        i32_times.push(i32_time);
        f64_times.push(f64_time);
        complex_times.push(complex_time);
        user_times.push(user_time);

        println!("Test {} done...", i+1);
    }

        let mut writer = csv::Writer::from_path("doubly_linked_list.csv").unwrap();
    writer.write_record(&["i32", "f64", "complex", "user"]).unwrap();
    for i in 0..10 {
        writer.write_record(&[i32_times[i].to_string(), f64_times[i].to_string(), complex_times[i].to_string(), user_times[i].to_string()]).unwrap();
    }
    writer.flush().unwrap();
}