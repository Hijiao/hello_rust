#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { qdata: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.qdata.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.qdata.pop()
    }
}

fn main() {
    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);

    println!("pop node: {:?}", q.pop());
    println!("pop node: {:?}", q.pop());
    println!("after pop{:?}", q);


    let n = q.pop();
    println!("{:?}", n);


}

#[test]
fn test_it(){

}