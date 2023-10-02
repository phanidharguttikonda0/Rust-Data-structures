use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
struct Node<T> {
    value:T,
    previous: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T>{
    pub fn new(value: T) -> Node<T>{
        Node {
            value,
            previous: None,
            next: None
        }
    }
}


struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>
}


impl<T: std::fmt::Debug + Clone> DoublyLinkedList<T> {

    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    // add the element at the back of the linked list
    pub fn add(&mut self,value: T) {
        let node = Rc::new(RefCell::new(Node::new(value))) ;
        match self.head.clone() {
            None => {
                self.head = Some(node.clone()) ;
                self.tail = Some(node.clone()) ;
            }
            Some(_) =>{
                self.tail.clone().unwrap().borrow_mut().next = Some(node.clone()) ;
                node.clone().borrow_mut().previous = self.tail.clone() ;
                self.tail = self.tail.clone().unwrap().clone().borrow_mut().next.clone() ;
            }
        }
    }


    pub fn iterate(&self) {
        let mut current = self.head.clone() ;
        while !current.is_none() {
            println!("{:?}", current.clone().unwrap().borrow_mut().value) ;
            current = current.clone().unwrap().borrow_mut().next.clone() ;
        }
    }

    pub fn add_front(&mut self, value:T) {
        let node = Rc::new(RefCell::new(Node::new(value))) ;
        match self.head.clone() {
            None => {
                self.head = Some(node.clone()) ;
                self.tail = Some(node.clone()) ;
            }
            Some(_) =>{
                self.head.clone().unwrap().borrow_mut().previous = Some(node.clone()) ;
                node.clone().borrow_mut().next = self.head.clone() ;
                self.head = Some(node.clone()) ;
            }
        }
    }

    pub fn delete_front(&mut self) {
        match self.head.clone() {
            None => println!("no Element"),
            Some(_) => {
                self.head = self.head.clone().unwrap().borrow_mut().next.clone() ;
                match self.head.clone() {
                    None => self.tail =None,
                    Some(_) => self.head.clone().unwrap().borrow_mut().previous = None ,
                }
            }
        } ;
    }

    pub fn delete_back(&mut self)  {
        match self.tail.clone() {
            None => println!("no elements to delete"),
            Some(_) => {
                match self.tail.clone().unwrap().borrow_mut().previous.clone() {
                    None => {
                        self.head =None ;
                        self.tail = None ;
                    }
                    Some(_) => {
                        self.tail = self.tail.clone().unwrap().borrow_mut().previous.clone()  ;
                        self.tail.clone().unwrap().borrow_mut().next = None ;
                    }
                }
            }
        } ;
    }

    pub fn peek(&self) -> Option<T> {
        match self.head.clone() {
            None => None,
            Some(_) => Some(self.head.clone().unwrap().borrow_mut().value.clone())
        }
    }

    pub fn last(&self) -> Option<T> {
        match self.tail.clone() {
            None => None,
            Some(_) => Some(self.tail.clone().unwrap().borrow_mut().value.clone())
        }
    }
}






fn main() {
    let mut doubly_list = DoublyLinkedList::new() ;
    doubly_list.add(5) ;
    doubly_list.iterate() ;
    doubly_list.add_front(7) ;
    println!("adding 7 front of the list") ;
    doubly_list.iterate() ;
    doubly_list.delete_front() ;
    println!("After deleting 7 from the front of the list") ;
    doubly_list.iterate() ;
    doubly_list.delete_back() ;
    doubly_list.iterate() ;
    println!("{:?}", doubly_list.peek()) ;
    println!("{:?}", doubly_list.last()) ;
    println!("adding elements to the list") ;
    doubly_list.add_front(5) ;
    doubly_list.add_front(4) ;
    doubly_list.add_front(3) ;
    doubly_list.add_front(2) ;
    doubly_list.add_front(1) ;
    doubly_list.add(10) ;
    println!("iterating over the list") ;
    doubly_list.iterate() ;
    println!("Peek Element is {:?}", doubly_list.peek().unwrap()) ;
    println!("last Element is {:?}", doubly_list.last().unwrap()) ;
}