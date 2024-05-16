
#[cfg(test)]
mod general_tests {
    use crate::linked_list::DoublyLinkedList;
    use super::*;

    #[test]
    fn test_push_and_pop_first() {
        let mut list = DoublyLinkedList::new();
        assert!(list.head.is_none());
        assert!(list.tail.is_none());

                list.push_first(1);
        assert_eq!(list.head.as_ref().unwrap().borrow().data, 1);

                assert_eq!(list.pop_first(), Some(1));
        assert!(list.head.is_none());
        assert!(list.tail.is_none());

                list.push_first(2);
        list.push_first(3);
        assert_eq!(list.pop_first(), Some(3));
        assert_eq!(list.head.as_ref().unwrap().borrow().data, 2);
    }

    #[test]
    fn test_push_and_pop_back() {
        let mut list = DoublyLinkedList::new();

                list.push_back(1);
        list.push_back(2);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, 2);

                assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert!(list.head.is_none());
        assert!(list.tail.is_none());

                list.push_back(3);
        list.push_back(4);
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.head.as_ref().unwrap().borrow().data, 3);
    }

    #[test]
    fn test_remove_middle() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

                let middle_node = list.head.as_ref().unwrap().borrow().next.clone().unwrap();
        list.remove(middle_node);

                assert_eq!(list.head.as_ref().unwrap().borrow().data, 1);
        assert_eq!(list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().data, 3);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, 3);
        assert_eq!(list.tail.as_ref().unwrap().borrow().prev.as_ref().unwrap().borrow().data, 1);
    }
}

#[cfg(test)]
mod insert_tests {
    use crate::enums::Side;
    use crate::linked_list::DoublyLinkedList;
    use super::*;

    #[test]
    fn test_insert_before() {
        let mut list = DoublyLinkedList::new();
        let first_item = 1;
        let second_item = 2;

                list.push_back(first_item);

                let head = list.head.clone().unwrap();
        list.insert(Side::Before, second_item, head);

                assert_eq!(list.head.as_ref().unwrap().borrow().data, second_item);
        assert_eq!(list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().data, first_item);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, first_item);
    }

    #[test]
    fn test_insert_after() {
        let mut list = DoublyLinkedList::new();
        let first_item = 1;
        let second_item = 2;

                list.push_back(first_item);

                let tail = list.tail.clone().unwrap();
        list.insert(Side::After, second_item, tail);

                assert_eq!(list.head.as_ref().unwrap().borrow().data, first_item);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, second_item);
        assert_eq!(list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().data, second_item);
    }

    #[test]
    fn test_insert_empty_list() {
        let mut list = DoublyLinkedList::new();
        let first_item = 1;

                assert!(list.head.is_none() && list.tail.is_none());

                list.push_back(first_item);
        assert_eq!(list.head.as_ref().unwrap().borrow().data, first_item);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, first_item);
    }

    #[test]
    fn test_insert_between_nodes() {
        let mut list = DoublyLinkedList::new();
        let first_item = 1;
        let second_item = 2;
        let third_item = 3;

                list.push_back(first_item);
        list.push_back(third_item);

                let first_node = list.head.clone().unwrap();
        list.insert(Side::After, second_item, first_node);

                assert_eq!(list.head.as_ref().unwrap().borrow().data, first_item);
        assert_eq!(list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().data, second_item);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, third_item);
        assert_eq!(list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().data, third_item);
    }
}

#[cfg(test)]
mod remove_tests {
    use crate::linked_list::DoublyLinkedList;
    use super::*;

    #[test]
    fn test_remove_single_element() {
        let mut list = DoublyLinkedList::new();
        list.push_first(10);

        let head = list.head.clone().unwrap();
        assert_eq!(list.remove(head), Some(10));
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_pop_first_single_element() {
        let mut list = DoublyLinkedList::new();
        list.push_first(10);
        assert_eq!(list.pop_first(), Some(10));
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_pop_back_single_element() {
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        assert_eq!(list.pop_back(), Some(10));
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_remove_multiple_elements() {
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

                let middle_node = list.head.as_ref().unwrap().borrow().next.clone().unwrap();
        assert_eq!(list.remove(middle_node), Some(20));

                assert_eq!(list.head.as_ref().unwrap().borrow().data, 10);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, 30);
        assert_eq!(list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().data, 30);
        assert_eq!(list.tail.as_ref().unwrap().borrow().prev.as_ref().unwrap().borrow().data, 10);
    }

    #[test]
    fn test_pop_first_multiple_elements() {
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert_eq!(list.pop_first(), Some(10));
                assert_eq!(list.head.as_ref().unwrap().borrow().data, 20);
        assert!(list.head.as_ref().unwrap().borrow().prev.is_none());
    }

    #[test]
    fn test_pop_back_multiple_elements() {
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert_eq!(list.pop_back(), Some(30));
                assert_eq!(list.tail.as_ref().unwrap().borrow().data, 20);
        assert!(list.tail.as_ref().unwrap().borrow().next.is_none());
    }
}

#[cfg(test)]
mod edge_case_tests {
    use crate::linked_list::DoublyLinkedList;
    use super::*;

    #[test]
    fn test_remove_head() {
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        list.push_back(20);

        let head = list.head.clone().unwrap();
        assert_eq!(list.remove(head), Some(10));
        assert_eq!(list.head.as_ref().unwrap().borrow().data, 20);
        assert!(list.head.as_ref().unwrap().borrow().prev.is_none());
    }

    #[test]
    fn test_remove_tail() {
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        list.push_back(20);

        let tail = list.tail.clone().unwrap();
        assert_eq!(list.remove(tail), Some(20));
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, 10);
        assert!(list.tail.as_ref().unwrap().borrow().next.is_none());
    }

    #[test]
    fn test_pop_first_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(list.pop_first(), None);
    }

    #[test]
    fn test_pop_back_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(list.pop_back(), None);
    }
}

