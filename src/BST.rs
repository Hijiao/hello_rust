type TreeNode<K, V> = Option<Box<Node<K, V>>>;

struct Node<K, V> {
    key: K,
    value: V,
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
}

trait BinaryTree<K, V> {
    fn pre_order(&self);
    fn in_order(&self);
    fn post_order(&self);
}

trait BinarySearchTree<K: PartialOrd, V>: BinaryTree<K, V> {
    fn insert(&mut self, key: K, val: V);
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Node<K, V> {
        Node {
            key: key,
            value: val,
            left: None,
            right: None,
        }
    }
}

impl<K: PartialOrd+std::fmt::Display, V:std::fmt::Display> BinarySearchTree<K, V> for Node<K, V> {
    fn insert(&mut self, key: K, val: V) {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                right.insert(key, val)
            } else {
                self.right = Some(Box::new(Node::new(key, val)));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(key, val);
            } else {
                self.left = Some(Box::new(Node::new(key, val)));
            }
        }
    }
}

impl<K:std::fmt::Display, V:std::fmt::Display> BinaryTree<K, V> for Node<K, V> {
    fn pre_order(&self) {
        println!("k:{},k:{}", self.key,self.value);

        if let Some(ref left) = self.left {
            left.pre_order();
        }

        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        println!("k:{},k:{}", self.key,self.value);

        if let Some(ref right) = self.right {
            right.in_order();
        }
    }

    fn post_order(&self) {
        if let Some(ref left) = self.left {
            left.post_order();
        }

        if let Some(ref right) = self.right {
            right.post_order();
        }
        println!("k:{},k:{}", self.key,self.value);
    }
}


type BST<K, V> = Node<K, V>;

fn test_insert() {
    let mut root = BST::<i32, i32>::new(3, 4);
    root.insert(2, 3);
    root.insert(4, 6);
    root.insert(5, 5);
    root.insert(6, 6);
    root.insert(1, 8);
    root.insert(1, 9);

    if let Some(ref left) = root.left {
        assert_eq!(left.value, 3);
    }

    if let Some(ref right) = root.right {
        assert_eq!(right.value, 6);
        if let Some(ref right) = right.right {
            assert_eq!(right.value, 5);
        }
    }
    println!("Pre Order traversal");
    root.pre_order();
    println!("In Order traversal");
    root.in_order();
    println!("Pos Order traversal");
    root.post_order();
}

fn main() {
    test_insert();
}