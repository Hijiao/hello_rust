use List::*;

enum List {
    // Cons: 包含一个元素和一个指向下一个节点的指针的元组结构
    Cons(u32, Box<List>),
    // Nil: 表示一个链表节点的末端
    Nil,
}

impl List{
    fn new() ->List{
        Nil
    }

    fn prepend(self,element:u32) ->List{
        Cons(element,Box::new(self))
    }

    fn stringify(&self)->String{
        match *self{
            Cons(head,ref tail)=>{
                format!("{}, {}", head, tail.stringify())
            }
            Nil=>{
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

//    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}