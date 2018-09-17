#[derive(Debug)]
struct Queue<T> {
    header: Option<Box<QueueNode<T>>>,
    tail: Option<Box<QueueNode<T>>>,
}

#[derive(Clone, Debug)]
struct QueueNode<T> {
    val: T,
    next: Option<Box<QueueNode<T>>>,
}

impl<T> QueueNode<T> {
    fn new(val: T) -> QueueNode<T> {
        QueueNode {
            val: val,
            next: None,
        }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            header: None,
            tail: None,
        }
    }
    pub fn push(&mut self, val: T) {
        let mut node = Some(Box::new(QueueNode::new(val)));

        match self.header {
            None => {
                self.header = node;
                self.tail = node;
            }
            Some(_) => {
                self.tail.next = node;
                self.tail = self.tail.next;
            }
        }
    }
}

#[test]
fn test_it() {
    let mut queue = Queue::<i32>::new();
    queue.push(2);
    queue.push(3);
    println!("{:?}", queue);
}