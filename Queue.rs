//* Dynamic Queue Data Structure  */

use std::cell::RefCell;
use std::rc::Rc;
#[derive(PartialEq)]
struct Node<T>{
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>
}


impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            next: None
        }
    }
}


struct Queue<T> {
    front: Option<Rc<RefCell<Node<T>>>>,
    rear: Option<Rc<RefCell<Node<T>>>>
}


impl<T: Clone + std::cmp::PartialEq + std::fmt::Debug>   Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            front: None,
            rear: None
        }
    }

    fn Enqueue(&mut self,value: T) {
        let node = Rc::new(RefCell::new(Node::new(value))) ;
        match self.rear.clone() {
            None => {
                self.front = Some(node.clone()) ;
                self.rear = Some(node.clone()) ;
            }
            Some(_) =>{
                self.rear.clone().unwrap().borrow_mut().next = Some(node.clone()) ;
                self.rear = Some(node.clone()) ;
            }
        }
    }

    fn Dequeue(&mut self) -> Option<T>{
        // deleting a node from the front end of the queue
        match self.front.clone() {
            None => None,
            Some(_) => {
                let value:T = self.front.clone().unwrap().borrow_mut().value.clone() ;
                if self.front.clone() == self.rear.clone() {
                    self.front = None ;
                    self.rear = None ;
                }else{
                    self.front = self.front.clone().unwrap().borrow_mut().next.clone() ;
                }
                Some(value)
            }
        }
    }

    fn Peek(&self) -> Option<T> {
        match self.front.clone() {
            None => None,
            Some(_) =>{
                Some(self.front.clone().unwrap().borrow_mut().value.clone())
            }
        }
    }

    fn isEmpty(&self) -> bool {
        match self.front.clone() {
            None => true,
            Some(_) => {
                false
            }
        }
    }

    fn iterate(&self) {
        let mut current = self.front.clone() ;
        while !current.is_none() {
            println!("{:?}", current.clone().unwrap().borrow_mut().value.clone()) ;
            current = current.clone().unwrap().borrow_mut().next.clone() ;
        }
    }
}

fn main() {
    println!("Queue follows first in first out Principle") ;
    // creating the Queue
    let mut queue = Queue::new() ;
    println!("we Created the Queue but no value intially it is empty let's check it {:?}", queue.isEmpty()) ;
    println!("Let's add some values into the queue") ;
    queue.Enqueue(1) ;
    queue.Enqueue(2) ;
    queue.Enqueue(3) ;
    queue.Enqueue(4) ;
    queue.Enqueue(5) ;
    queue.iterate() ;
    println!("Deleting Elements from the queue") ;
    queue.Dequeue() ;
    queue.Dequeue() ;
    println!("The Remaining Elements in the Queue") ;
    queue.iterate() ;
    println!("The top element is {:?}", queue.Peek()) ;
    println!("queue is not Empty let's Check it {:?}", queue.isEmpty()) ;
}