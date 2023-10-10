use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug ;

pub trait  TreeIterators<T : Debug> {
    fn pre_order(&self, root: Option<Rc<RefCell<Node<T>>>>) {
        if root.clone().is_none() { return }
        println!("{:?}", root.clone().unwrap().borrow_mut().data) ;
        self.pre_order(root.clone().unwrap().borrow_mut().left.clone()) ;
        self.pre_order(root.clone().unwrap().borrow_mut().right.clone()) ;
    }
    fn in_order(&self, root: Option<Rc<RefCell<Node<T>>>>) {
        if root.clone().is_none() {return}
        self.in_order(root.clone().unwrap().borrow_mut().left.clone()) ;
        println!("{:?}", root.clone().unwrap().borrow_mut().data) ;
        self.in_order(root.clone().unwrap().borrow_mut().right.clone()) ;
    }
    fn post_order(&self, root: Option<Rc<RefCell<Node<T>>>>) {
        if root.clone().is_none() {return}
        self.post_order(root.clone().unwrap().borrow_mut().left.clone()) ;
        self.post_order(root.clone().unwrap().borrow_mut().right.clone()) ;
        println!("{:?}", root.clone().unwrap().borrow_mut().data) ;
    }
}
pub struct Node<T> {
    data : T,
    left : Option<Rc<RefCell<Node<T>>>>,
    right : Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            data : value,
            left : None,
            right : None
        }
    }
}

struct  BST<T> {
    root : Option<Rc<RefCell<Node<T>>>>
}


impl<T:  std::cmp::PartialOrd + Clone>  BST<T> {
    fn new() -> BST<T> {
        BST {
            root : None
        }
    }

    fn add(&mut self,value: T) {
        // adding a value to the BST
        let node = Rc::new(RefCell::new(Node::new(value))) ;
        match self.root.clone() {
            None => self.root = Some(node.clone()) ,
            Some(_) => {
                let mut current = self.root.clone() ;
                loop {
                    if current.clone().unwrap().borrow_mut().data > node.clone().borrow_mut().data {
                        // we have to move to the left side
                        if current.clone().unwrap().borrow_mut().left.clone().is_none() {
                            current.clone().unwrap().borrow_mut().left = Some(node.clone()) ;
                            break ;
                        }
                        current = current.clone().unwrap().borrow_mut().left.clone() ;
                    }else if current.clone().unwrap().borrow_mut().data < node.clone().borrow_mut().data {
                        // we have to move towards the right side of the value
                        if current.clone().unwrap().borrow_mut().right.clone().is_none() {
                            current.clone().unwrap().borrow_mut().right = Some(node.clone()) ;
                            break ;
                        }
                        current = current.clone().unwrap().borrow_mut().right.clone() ;
                    }else{
                        // If we find a duplicate we will break the statement
                        break ;
                    }
                }
            }
        }
    }


    // fn delete(&mut self,value: T) {
    //     // deleting an element from the BST
    // }

    fn root_element(&self) -> Option<T> {
        Some(self.root.clone().unwrap().borrow_mut().data.clone())
    }

    // add more functions like height of the bst and finding min and max element from bst
}

impl<T: Debug> TreeIterators<T> for BST<T> {}

fn main() {
    let mut bst = BST::new() ;
    bst.add(5) ;
    bst.add(1) ;
    bst.add(3) ;
    bst.add(2) ;
    bst.add(7) ;
    bst.add(9) ;
    bst.add(8) ;
    bst.pre_order(bst.root.clone()) ;
}