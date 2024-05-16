use crate::enums::Side;
use crate::linked_list::DoublyLinkedList;

#[cfg(test)]
mod i32_tests {
    use super::*;

    #[test]
    fn test_push_and_pop_back() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);
        assert!(list.pop_back().is_none());

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back().unwrap(), 3);
        assert_eq!(list.pop_back().unwrap(), 2);
        assert_eq!(list.pop_back().unwrap(), 1);
        assert!(list.pop_back().is_none());
    }

    #[test]
    fn test_push_and_pop_first() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);
        assert!(list.pop_first().is_none());

        list.push_first(1);
        list.push_first(2);
        list.push_first(3);

        assert_eq!(list.pop_first().unwrap(), 3);
        assert_eq!(list.pop_first().unwrap(), 2);
        assert_eq!(list.pop_first().unwrap(), 1);
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_insert_nodes() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        let anchor = list.head.clone().unwrap();

        list.insert(Side::After, 2, anchor.clone());
        list.insert(Side::Before, 0, anchor);

        assert_eq!(list.pop_first().unwrap(), 0);
        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 2);
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_remove_nodes() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let node_to_remove = list.head.as_ref().unwrap().borrow().next.clone().unwrap();
        list.remove(node_to_remove);

        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 3);
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_node_pool_efficiency() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.init(3, 0);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        list.pop_back();
        list.pop_back();

        list.push_back(5);
        list.push_back(6);

        assert_eq!(list.pop_back().unwrap(), 6);
        assert_eq!(list.pop_back().unwrap(), 5);
        assert_eq!(list.pop_back().unwrap(), 2);
        assert_eq!(list.pop_back().unwrap(), 1);
        assert!(list.pop_back().is_none());
    }
}

#[cfg(test)]
mod complex_struct_tests {
    use super::*;
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: &str, age: u32) -> Self {
            Person {
                name: name.to_string(),
                age,
            }
        }
    }

    #[test]
    fn test_complex_push_and_pop() {
        let mut list = DoublyLinkedList::new();
        list.init(5, Person::new("Default", 0));

        list.push_back(Person::new("Alice", 30));
        list.push_back(Person::new("Bob", 25));
        list.push_back(Person::new("Charlie", 35));

        assert_eq!(list.pop_back().unwrap(), Person::new("Charlie", 35));
        assert_eq!(list.pop_back().unwrap(), Person::new("Bob", 25));
        assert_eq!(list.pop_back().unwrap(), Person::new("Alice", 30));
        assert!(list.pop_back().is_none());
    }

    #[test]
    fn test_complex_insert() {
        let mut list = DoublyLinkedList::new();
        list.init(5, Person::new("Default", 0));

        list.push_back(Person::new("Alice", 30));
        let anchor = list.head.clone().unwrap();

        list.insert(Side::After, Person::new("Charlie", 35), anchor.clone());
        list.insert(Side::Before, Person::new("Bob", 25), anchor);

        assert_eq!(list.pop_first().unwrap(), Person::new("Bob", 25));
        assert_eq!(list.pop_first().unwrap(), Person::new("Alice", 30));
        assert_eq!(list.pop_first().unwrap(), Person::new("Charlie", 35));
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_complex_remove() {
        let mut list = DoublyLinkedList::new();
        list.init(5, Person::new("Default", 0));

        list.push_back(Person::new("Alice", 30));
        list.push_back(Person::new("Bob", 25));
        list.push_back(Person::new("Charlie", 35));

        let node_to_remove = list.head.as_ref().unwrap().borrow().next.clone().unwrap();
        list.remove(node_to_remove);

        assert_eq!(list.pop_first().unwrap(), Person::new("Alice", 30));
        assert_eq!(list.pop_first().unwrap(), Person::new("Charlie", 35));
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_node_pool_efficiency_with_complex_struct() {
        let mut list = DoublyLinkedList::<Person>::new();
        list.init(3, Person::new("Default", 0));

        list.push_back(Person::new("Alice", 30));
        list.push_back(Person::new("Bob", 25));
        list.push_back(Person::new("Charlie", 35));
        list.push_back(Person::new("Diana", 40));

        list.pop_back();
        list.pop_back();

        list.push_back(Person::new("Eve", 45));
        list.push_back(Person::new("Frank", 50));

        assert_eq!(list.pop_back().unwrap(), Person::new("Frank", 50));
        assert_eq!(list.pop_back().unwrap(), Person::new("Eve", 45));
        assert_eq!(list.pop_back().unwrap(), Person::new("Bob", 25));
        assert_eq!(list.pop_back().unwrap(), Person::new("Alice", 30));
        assert!(list.pop_back().is_none());
    }
}

#[cfg(test)]
mod insert_tests {
    use super::*;

    #[test]
    fn test_insert_after() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        let anchor = list.head.clone().unwrap();

        list.insert(Side::After, 2, anchor.clone());
        list.insert(Side::After, 3, anchor.clone());

        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 3);
        assert_eq!(list.pop_first().unwrap(), 2);
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_insert_before() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        let anchor = list.head.clone().unwrap();

        list.insert(Side::Before, 2, anchor.clone());
        list.insert(Side::Before, 3, anchor.clone());

        assert_eq!(list.pop_first().unwrap(), 2);
        assert_eq!(list.pop_first().unwrap(), 3);
        assert_eq!(list.pop_first().unwrap(), 1);
        assert!(list.pop_first().is_none());
    }
}

#[cfg(test)]
mod remove_tests {
    use super::*;

    #[test]
    fn test_remove_middle() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let node_to_remove = list.head.as_ref().unwrap().borrow().next.clone().unwrap();
        list.remove(node_to_remove);

        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 3);
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_remove_first() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let node_to_remove = list.head.clone().unwrap();
        list.remove(node_to_remove);

        assert_eq!(list.pop_first().unwrap(), 2);
        assert_eq!(list.pop_first().unwrap(), 3);
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_remove_last() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let node_to_remove = list.tail.clone().unwrap();
        list.remove(node_to_remove);

        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 2);
        assert!(list.pop_first().is_none());
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        assert!(list.pop_first().is_none());
        assert!(list.pop_back().is_none());
    }

    #[test]
    fn test_single_element_list() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);

        assert_eq!(list.pop_first().unwrap(), 1);
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_single_element_list_pop_back() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);

        assert_eq!(list.pop_back().unwrap(), 1);
        assert!(list.pop_back().is_none());
    }

    #[test]
    fn test_single_element_list_insert() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        let anchor = list.head.clone().unwrap();

        list.insert(Side::After, 2, anchor.clone());
        list.insert(Side::Before, 0, anchor);

        assert_eq!(list.pop_first().unwrap(), 0);
        assert_eq!(list.pop_first().unwrap(), 1);
        assert_eq!(list.pop_first().unwrap(), 2);
        assert!(list.pop_first().is_none());
    }

    #[test]
    fn test_single_element_list_remove() {
        let mut list = DoublyLinkedList::new();
        list.init(5, 0);

        list.push_back(1);
        let node_to_remove = list.head.clone().unwrap();
        list.remove(node_to_remove);

        assert!(list.pop_first().is_none());
    }
}