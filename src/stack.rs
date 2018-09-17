
#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>
}

#[derive(Clone, Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode {
            val: val,
            next: None,
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            top: None
        }
    }

    pub fn push(&mut self, val: T) {
        let next = self.top.take();
        let mut node = StackNode::new(val);
        node.next = next;
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.top.take();
        match node {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}


#[test]
fn test_it() {
    println!("Hello, world!");
    let a = 5;
    let b = 9;

    let mut s = Stack::<i32>::new();
    assert_eq!(s.pop(), None);

    s.push(a);
    s.push(b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(b));
    assert_eq!(s.pop(), Some(a));
    assert_eq!(s.pop(), None);
}