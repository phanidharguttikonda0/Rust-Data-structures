use std::cell::RefCell;
use std::rc::Rc;



struct Node<T> {
    value : T,
    left : Option<Rc<RefCell<Node<T>>>>,
    right : Option<Rc<RefCell<Node<T>>>>
}


impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left : None,
            right : None
        }
    }
}


struct BinarySearchTree<T>{
    root : Option<Rc<RefCell<Node<T>>>>
}


// there should be no duplicates in this BST
impl<T: std::cmp::PartialOrd + std::fmt::Debug + Clone> BinarySearchTree<T>  {

    fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            root : None
        }
    }

    fn add(&mut self,value: T) {
        let node = Rc::new(RefCell::new(Node::new(value.clone()))) ;
        match self.root.clone() {
            None => self.root = Some(node.clone()) ,
            Some(_) => {
                // check for a place where the element exist
                let mut current = self.root.clone() ;
                while !current.clone().is_none() {
                    if current.clone().unwrap().borrow_mut().value.clone() > value {
                        if current.clone().unwrap().borrow_mut().left.is_none() {
                            current.clone().unwrap().borrow_mut().left = Some(node.clone()) ;
                            break ;
                        }else{
                            current = current.clone().unwrap().borrow_mut().left.clone() ;
                        }
                    }else if current.clone().unwrap().borrow_mut().value.clone() < value {
                        if current.clone().unwrap().borrow_mut().right.is_none() {
                            current.clone().unwrap().borrow_mut().right = Some(node.clone()) ;
                            break ;
                        }else{
                            current = current.clone().unwrap().borrow_mut().right.clone() ;
                        }
                    }else{
                        break; // no duplicates are allowed
                    }
                }
            }
        }
    }

    fn inorder_traversal(&self, root:Option<Rc<RefCell<Node<T>>>>) {
        if root.is_none() { return }
        self.inorder_traversal(root.clone().unwrap().borrow_mut().left.clone()) ;
        println!("{:?}", root.clone().unwrap().borrow_mut().value) ;
        self.inorder_traversal(root.clone().unwrap().borrow_mut().right.clone()) ;
        return ;
    }

    fn preorder_traversal(&self, root:Option<Rc<RefCell<Node<T>>>>) {
        if root.is_none() { return }
        println!("{:?}", root.clone().unwrap().borrow_mut().value) ;
        self.preorder_traversal(root.clone().unwrap().borrow_mut().left.clone()) ;
        self.preorder_traversal(root.clone().unwrap().borrow_mut().right.clone()) ;
        return ;
    }

    fn postorder_traversal(&self, root:Option<Rc<RefCell<Node<T>>>>) {
        if root.is_none() { return }
        self.postorder_traversal(root.clone().unwrap().borrow_mut().left.clone()) ;
        self.postorder_traversal(root.clone().unwrap().borrow_mut().right.clone()) ;
        println!("{:?}", root.clone().unwrap().borrow_mut().value) ;
        return ;
    }


}



/*

Duplicates are not allowed if you want to manage duplicates you can add features to the code
*/
fn main() {
    let mut bst = BinarySearchTree::new() ;
    bst.add(7) ;
    bst.add(2) ;
    bst.add(5) ;
    bst.add(12) ;
    bst.add(9) ;
    println!("Inorder-Traversal") ;
    bst.inorder_traversal(bst.root.clone()) ;
    println!("Pre-Order Traversal") ;
    bst.preorder_traversal(bst.root.clone()) ;
    println!("Post-Order Traversal") ;
    bst.postorder_traversal(bst.root.clone()) ;
}
