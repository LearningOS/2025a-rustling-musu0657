/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

use std::ptr::NonNull;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct LinkedList<T> {
    start: Option<NonNull<Node<T>>>,
    nodes: Vec<Box<Node<T>>>,
    length: usize,
}

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        Self {
            start: None,
            nodes: Vec::new(),
            length: 0,
        }
    }

    pub fn add(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: None,
        });
        let non_null_node = NonNull::new(Box::into_raw(new_node)).unwrap();

        if self.start.is_none() {
            self.start = Some(non_null_node);
        } else {
            let mut current = self.start;
            while let Some(ptr) = current {
                let next_ptr = unsafe { ptr.as_ref().next };
                if next_ptr.is_none() {
                    unsafe { ptr.as_mut().next = Some(non_null_node) };
                    break;
                }
                current = next_ptr;
            }
        }
        self.nodes.push(Box::from_raw(non_null_node.as_ptr()));
        self.length += 1;
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged = LinkedList::new();
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;

        while ptr_a.is_some() && ptr_b.is_some() {
            let val_a = unsafe { ptr_a.unwrap().as_ref().val.clone() };
            let val_b = unsafe { ptr_b.unwrap().as_ref().val.clone() };

            if val_a <= val_b {
                merged.add(val_a);
                ptr_a = unsafe { ptr_a.unwrap().as_ref().next };
            } else {
                merged.add(val_b);
                ptr_b = unsafe { ptr_b.unwrap().as_ref().next };
            }
        }

        while ptr_a.is_some() {
            let val = unsafe { ptr_a.unwrap().as_ref().val.clone() };
            merged.add(val);
            ptr_a = unsafe { ptr_a.unwrap().as_ref().next };
        }

        while ptr_b.is_some() {
            let val = unsafe { ptr_b.unwrap().as_ref().val.clone() };
            merged.add(val);
            ptr_b = unsafe { ptr_b.unwrap().as_ref().next };
        }

        merged
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut current = self.start;
        let mut first = true;
        while let Some(node_ptr) = current {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            let node = unsafe { node_ptr.as_ref() };
            write!(f, "{}", node.val)?;
            current = node.next;
        }
        Ok(())
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}