use std::borrow::Cow;


#[derive(Debug)]
struct Queue<'a, T: 'a + Copy> {
    header: Option<Cow<'a, QueueNode<'a, T>>>,
    tail: Option<Cow<'a, QueueNode<'a, T>>>,
}

#[derive(Clone, Debug)]
struct QueueNode<'a, T: 'a + Copy> {
    val: T,
    next: Option<Cow<'a, QueueNode<'a, T>>>,
}

impl<'a, 'b:'a, T: Copy> QueueNode<'a, T> {
    fn new(val:  T) -> QueueNode<'a, T> {
        QueueNode {
            val: val,
            next: None,
        }
    }
}

impl<'a, T: 'a + Copy> Queue<'a, T> {
    pub fn new() -> Queue<'a, T> {
        Queue {
            header: None,
            tail: None,
        }
    }
    pub fn push<'b:'a>(&'b mut self, val:&'b T) -> Option<Cow<'a,QueueNode<T>>> {
        let node = Cow::Owned(QueueNode::new(*val));
        match self.tail {
            None => {
                self.header = Some(Cow::Borrowed(&node));
                self.tail = Some(Cow::Borrowed(&node));
            }
            Some(pre) => {
//                let mut next = node.next.unwrap().downcast_mut();
//                next = node.clone();
                (pre).next = Some(Cow::Borrowed(&node));
//                node=None;
            }
        };

        Some(node)

//        match  self.tail {
//            None =>{
//                self.header = Some(node);
//                self.tail = Some(node);
//            }
//            Some(ref mut  t )=>{
//                t.next = Some(node);
//                self.tail =Some(node);
//            }
//        }
    }
}

fn main() {
    let mut queue = Queue::<str>::new();
    queue.push("a");
    queue.push("b");

    println!("{:?}", queue);
}