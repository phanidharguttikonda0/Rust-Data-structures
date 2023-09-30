use std::cell::RefCell;
use std::rc::Rc;


/*

Linked List with the Rc and RefCell Pointer

*/




#[derive(Debug)]
struct Node<T>{
    value : T,
    next : Option<Rc<RefCell<Node<T>>>>
}


impl<T> Node<T>{
    fn new(value: T) -> Node<T> {
        Node {
            value,
            next: None
        }
    }
}



struct LinkedList<T>{
    head :  Option<Rc<RefCell<Node<T>>>>,
    tail :  Option<Rc<RefCell<Node<T>>>>
}



impl<T : std::fmt::Debug + std::cmp::PartialEq + Clone>  LinkedList<T>{
    fn new() -> LinkedList<T> {
        LinkedList{
            head : None,
            tail : None,
        }
    }

    // adding the element at back of the linked list
    fn add(&mut self ,value: T) {
        let  node = Rc::new(RefCell::new(Node::new(value))) ;
       match self.head.clone() {
           None => {
               self.head = Some(node.clone()) ;
               self.tail = Some(node.clone()) ;
           }
           Some(_) => {
               self.tail.clone().unwrap().borrow_mut().next = Some(node.clone()) ;
               self.tail = self.tail.clone().unwrap().borrow_mut().next.clone() ;
           }
       }
    }

    fn iteration(&mut self) {
        let mut current = self.head.clone() ;
        while !current.is_none() {
            // unwrap() method gets the value of the Option enum
            println!("{:?}", current.clone().unwrap().borrow_mut().value) ;
            current = current.unwrap().borrow_mut().next.clone() ; // here we should clone the value
            // otherwise the value get's borrowed and the reference will be decreased in rc
        }

    }

    // deleting an element back of th e linkedlist
    fn pop(&mut self) {
        // as it is a singly linked list we have to iterate over the last second element
        let mut current = self.head.clone() ;
        if current.is_none() {}

        let mut previous = self.head.clone() ;
        while !current.clone().unwrap().borrow_mut().next.clone().is_none() {
            previous = current.clone() ;
            current = current.unwrap().borrow_mut().next.clone() ;
        }
        self.tail = previous.clone() ;
    }

    // deleting an element at the front of the linked list
    fn delete_front(&mut self) {
        // we need to clone the present element and also the next element
        // while accessing them
        self.head = self.head.clone().unwrap().borrow_mut().next.clone() ;
    }

    // deleting the element at any where in the linked list
    fn delete(&mut self, element:T) {

        // element to be deleted
        let mut current = self.head.clone() ;
        let mut previous = self.head.clone() ;

        // if head is the element to delete
        if current.clone().unwrap().borrow_mut().value == element {
            if self.head.clone().unwrap().borrow_mut().value == self.tail.clone().unwrap().borrow_mut().value{
                self.head = None ;
                self.tail = None ;
                return
            }
            self.head = current.clone().unwrap().borrow_mut().next.clone() ;
            return
        }

        while current.clone().unwrap().borrow_mut().value != element &&
            !current.is_none(){
            previous = current.clone() ;
            current = current.unwrap().borrow_mut().next.clone() ;
        }

        // it the deleting element is tail element
        if element == self.tail.clone().unwrap().borrow_mut().value {
            previous.clone().unwrap().borrow_mut().next = None ;
            self.tail = previous.clone() ; // we should clone other wise previous node of this previous
            // the next pointer losses
            println!("Executed") ;
        }
        else if !current.is_none(){
            previous.unwrap().borrow_mut().next = current.unwrap().borrow_mut().next.clone() ;
        }

        // we have been deleted an certain element
    }

    // adding the element at the front of the list
    fn add_front(&mut self, value:T) {
        let  node = Rc::new(RefCell::new(Node::new(value))) ;
        match self.tail.clone() {
            None => {
                self.head = Some(node.clone()) ;
                self.tail = Some(node.clone()) ;
            }
            Some(_) => {
                // we are going to add at the first of the list
                node.clone().borrow_mut().next = self.head.clone() ;
                self.head = Some(node.clone()) ;
            }
        }
    }

    fn peek(&self) -> T {

        return self.head.clone().unwrap().borrow_mut().value.clone() ;
    }

    fn last(&self) -> T{
        return self.tail.clone().unwrap().borrow_mut().value.clone() ;
    }

}






fn main() {
    let mut list = LinkedList::new() ;
    list.add(3) ;
    list.add(2) ;
    list.add(1) ;
    list.iteration() ;
    println!("iterating over the list again") ;
    list.iteration() ;
    list.pop() ; // removing the last element
    println!("removed the last element and iterating over again") ;
    list.iteration() ; // iterating those elements again
    // pushing some elements
    list.add(1) ;
    list.add(0) ;
    println!("iterating after the elements are added in to the list") ;
    list.iteration() ;
    // deleting front element
    println!("Deleting the front element and iterating over the list") ;
    list.delete_front() ;
    list.iteration() ;
    println!("Adding some more elements in to the list") ;
    list.add(5) ;
    list.add(4) ;
    list.add(3) ;
    list.iteration() ;
    println!("deleting an element 5 from the list") ;
    list.delete(5) ;
    list.iteration() ;
    println!("Deleting the last element using the delete method") ;
    list.delete(3) ;
    list.iteration() ;
    println!("Deleting the starting element") ;
    list.delete(2) ;
    list.iteration() ;
    println!("Adding the element to the first of the linked list") ;
    list.add_front(100) ;
    list.iteration() ;
    println!("The peek Element was {} ", list.peek()) ;
    println!("The last Element was {} ", list.last()) ;
    list.iteration() ;
}

/*

Running the code the rust compiler should be installed in our computer/pc/laptop

>> rustc LinkedList.rs
>> LinkedList

By running the above two commands we will get the out-put
*/