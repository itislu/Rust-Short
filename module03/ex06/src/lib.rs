use std::boxed;

/**
* allowed symbols:
   std::boxed::Box  std::option::Option
   std::panic
*/

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push_front(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }
    fn push_back(&mut self, value: T) {

        // if let Some(test) = self.head {
        //     let mut last = test;
        // }

        // let mut last = &self.head;
        // loop {
        //     if let Some(node) = last {
        //         last = &node.next;
        //     } else {
        //         break
        //     }
        // }
        // last.unwrap().next = Some(Box::new(Node {
        //     value,
        //     next: None,
        // }))

        let mut last: Option<&mut Box<Node<T>>> = None;

        for node in &mut self.head {
            last = Some(node);
        }

        if let Some(node) = last {
            node.next = Some(Box::new(Node {
                value,
                next: None,
            }))
        }

        // if let Some(last) = &self.head {
        //     // let last = node;
        //     while let Some(next) = last.next {
        //         last = &next;
        //     }
        //     last.next = Some(Box::new(Node {
        //         value,
        //         next: None,
        //     }))
        // }


        // let mut last = self.head;

        // for node in &last {
        //     last = Some(node);
        // }
        // if let Some(mut end) = last {
        //     end.next = Some(Box::new(Node {
        //         value,
        //         next: None,
        //     }))
        // }
    }

    fn count(&self) -> usize {
        let mut size: usize = 0;
        for _ in &self.head {
            size += 1;
        }
        size
    }

    fn get(&self, i: usize) -> Option<&T> {}
    fn get_mut(&mut self, i: usize) -> Option<&mut T> {}

    fn remove_front(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            let value = head.value;
            self.head = head.next;
            return Some(value);
        } else {
            None
        }
    }
    fn remove_back(&mut self) -> Option<T> {}
    fn clear(&mut self) {}
}

#[cfg(test)]
#[test]
fn default_list_is_empty() {
    let list: List<i32> = Default::default();
    assert_eq!(list.count(), 0);
}

#[cfg(test)]
#[test]
fn cloned_list_are_equal() {
    let mut list = List::new();
    list.push_back(String::from("Hello"));
    list.push_back(String::from("World"));

    let cloned = list.clone();
    assert_eq!(cloned.count(), list.count());
    assert_eq!(&cloned[0], &list[0]);
    assert_eq!(&cloned[1], &list[1]);
}

#[cfg(test)]
#[test]
#[should_panic(expected = "tried to access out of bound index 10")]
fn out_of_bound_access_panics() {
    let mut list: List<u32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list[10], 42);
}
