use std::cell::{Ref, RefCell};
use std::rc::Rc;

// we are going to create a stack data structure which follows first in last out principle
// we can create stack in two ways one by using an array and also by using the Node object dynamically
struct Node<T> {
    value : T,
    previous : Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            previous: None
        }
    }
}


struct Stack<T> {
    top : Option<Rc<RefCell<Node<T>>>>,
    size : u32
}


impl<T: Clone + std::fmt::Debug> Stack<T> {
    fn new() -> Stack<T>{
        Stack {
            top: None,
            size: 0
        }
    }

    fn push(&mut self,value: T) {
        let node = Rc::new(RefCell::new(Node::new(value))) ;
        match self.top.clone() {
            None => self.top = Some(node.clone()) ,
            Some(_) => {
                node.clone().borrow_mut().previous = self.top.clone() ;
                self.top = Some(node.clone()) ;
            }
        }
    }

    fn pop(&mut self) -> Option<T>{
        match self.top.clone() {
            None => None,
            Some(_) => {
                let value = Some(self.top.clone().unwrap().borrow_mut().value.clone()) ;
                self.top = self.top.clone().unwrap().borrow_mut().previous.clone() ;
                value
            }
        }
    }

    fn peek(&self) -> Option<T> {
        match self.top.clone() {
            None => None,
            Some(_) => Some(self.top.clone().unwrap().borrow_mut().value.clone())
        }
    }


    fn isEmpty(&self) -> bool {
        self.size == 0
    }

    fn iterator(&mut self) {
        let mut current = self.top.clone() ;
        while !current.is_none() {
            println!("{:?}", current.clone().unwrap().borrow_mut().value.clone()) ;
            current = current.clone().unwrap().borrow_mut().previous.clone() ;
        }
    }


}


fn main() {
    let mut stack = Stack::new() ;
    stack.push(5) ;
    stack.push(4) ;
    stack.push(3) ;
    stack.push(2) ;
    stack.push(1) ;
    println!("The elements should get in reverse order") ;
    stack.iterator() ;
    println!("poping out two elements") ;
    stack.pop() ;
    stack.pop() ;
    stack.iterator() ;
    println!("peek element is {:?}", stack.peek()) ;
    println!("poping out all elements") ;
    stack.pop() ;
    stack.pop() ;
    stack.pop() ;
    println!("Checking is stack empty {:?}", stack.isEmpty()) ;
}