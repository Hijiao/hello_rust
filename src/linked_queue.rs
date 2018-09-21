#![feature(box_into_raw_non_null)]

use std::ptr::NonNull;
use std::fmt;


#[derive(Clone, Debug)]
struct QueueNode<T: std::fmt::Debug> {
    val: T,
    next: Option<NonNull<QueueNode<T>>>,
}

impl<T: std::fmt::Debug> QueueNode<T> {
    fn new(val: T) -> Self {
        QueueNode {
            val,
            next: None,
        }
    }
}

impl<T: std::fmt::Debug> fmt::Display for QueueNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "val:{:?},next:{:?}", self.val, self.next)
    }
}


#[derive(Debug)]
struct Queue<T: std::fmt::Debug> {
    header: Option<NonNull<QueueNode<T>>>,
    tail: Option<NonNull<QueueNode<T>>>,
    len: usize,

}

impl<T: std::fmt::Debug> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            header: None,
            tail: None,
            len: 0,
        }
    }

    fn push_back_element(&mut self, val: T) {
        unsafe {
            let raw_node = Box::new(QueueNode::new(val));
//        println!(" raw_node :{:?}", raw_node );
//        println!(" raw_node :{:?}", &raw_node as *const QueueNode<T>);

            let node = Some(Box::into_raw_non_null(raw_node));
//            let node = NonNull::new(&mut raw_node as *mut QueueNode<T>);
//            let node = NonNull::new(raw_node.as_mut());
//        {
//            println!(" ===> node :{:?}", unsafe { &node.unwrap().as_ref() });
//        }
//        node.map(move|n|{println!("{:?}",n);});

//            println!("node in push back element:{:?}", node.unwrap().as_ref());

            match self.tail {
                None => {
                    self.header = node;
                }
                Some(mut tail) => {
                    tail.as_mut().next = node;
//                    println!("node :{:?}", unsafe { tail.as_ref() });
                }
            }

            self.len += 1;
            self.tail = node;
//            println!(" ===> header :{:?}", self.header.unwrap().as_ref());
        }
    }
    //
//    fn push_back(&mut self,mut node: Box<QueueNode<T>>){
//        unsafe {
//            let node = Some(Box::into_raw_non_null(node));
//
//            println!("node in push back:{:?}",node.unwrap().as_ref());
//
//            match self.tail {
//                None => self.header = node,
//                Some(mut tail) => tail.as_mut().next = node,
//            }
//            self.tail = node;
//            self.len += 1;
//        }
//    }
//
//    fn push(&mut self,val:T){
//        self.push_back(Box::new( QueueNode::new(val)));
//    }

    fn pop_header(&mut self) -> Option<T> {
        match self.header {
            None => None,
            Some(_) => {
                let val = unsafe { Box::from_raw(self.header.unwrap().as_ptr()) };
                self.header = val.next;
                Some(val.val)
            }
        }
    }
}

fn test_it() {
    let mut queue = Queue::<i32>::new();
    queue.push_back_element(2);
    queue.push_back_element(3);


    println!("queue:{:?}", queue);
    println!("queue.len:{:?}", queue.len);


    println!("queue.header:{:?} ", unsafe { queue.header.unwrap().as_ref() });
    println!("queue.tail:{:?} ", unsafe { &queue.tail.unwrap().as_ref() });

    let p = queue.pop_header();
    assert_eq!(p, Some(2));

    match queue.header {
        Some(raw_node) => {
            println!("header after pop: {}", unsafe { (raw_node.as_ref()) })
        }
        None => {
            println!("header after pop: None");
        }
    }


    let p = queue.pop_header();
    assert_eq!(p, Some(3));

    let p = queue.pop_header();
    assert_eq!(p, None);
}

fn main() {
////    test_it();
//    let  mut raw_node = QueueNode::new(2);
////        println!(" raw_node :{:?}", unsafe { *& raw_node });
//    println!(" raw_node :{:?}", &raw_node as *const QueueNode<i32>);
//
//
//     raw_node = QueueNode::new(3);
////        println!(" raw_node :{:?}", unsafe { *& raw_node });
//    println!(" raw_node :{:?}", & raw_node as *const QueueNode<i32>);

    test_it();
}