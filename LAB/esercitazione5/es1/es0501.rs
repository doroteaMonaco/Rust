#![allow(warnings)]
use std::mem;

pub mod mem_inspect {

    // dump object info:
    // size, address, bytes
    pub fn dump_object<T>(obj: &T) {
        let ptr = obj as *const T as *const u8;
        let _size = size_of::<T>();
        let _ptr = ptr as usize;
        println!("Object size: {_size}; address: {_ptr:x}");

        dump_memory(ptr, _size);
    }

    // dump memory info
    pub fn dump_memory(start: *const u8, size: usize) {
        let bytes = unsafe { std::slice::from_raw_parts(start, size) };

        println!("Bytes:");
        for (i, byte) in bytes.iter().enumerate() {
            print!("{:02x} ", byte);
            if i % 8 == 7 {
                println!();
            }
        }
        println!()
    }

    #[test]
    fn dump_object_example() {
        let s = "hello".to_string();
        dump_object(&s);

        let b = Box::new(s);
        // before running try to answer:
        // 1. what is the size of b?
        // 2. what is the content of b?
        dump_object(&b);

        // how to the the pointer of the wrapped object?
        let ptr = b.as_ref() as *const String as *const u8;
        println!("Pointer: {ptr:?}");

        assert!(true);
    }
}


pub mod List1 {
    use std::mem;


    #[derive(Clone)]
    pub enum Node<T> {
        Cons(T, Box<Node<T>>),
        Nil,
    }

    pub struct List<T> {
        head: Node<T>,
    }

    impl<T: Clone> List<T> {
        pub fn new() -> Self {
            List{ head: Node::Nil}
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // problem:
        // 1. you need to build a new list Node (elem: elem, self.head)
        // 2. but you can't move self.head, because self.head would be undefined
        // 3. you can't copy it either, because Box can't be copied
        // solution: use mem::replace to move the value of self.head into a new variable and replace it with Nil
        // 4. let self.head point to the new created node
        pub fn push(&mut self, elem: T) {
            let old_head = std::mem::replace(&mut self.head, Node::Nil);
            self.head = Node::Cons(elem, Box::new(old_head));
        }

        // pop the first element of the list and return it
        fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, Node::Nil) {
                Node::Cons(elem, next) => {
                    self.head = *next;
                    Some(elem)
                }
            Node::Nil => None,
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                Node::Cons(elem, next) => Some(&elem),  
                Node::Nil => None,
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        fn iter(&self) -> ListIter<T> {
            ListIter {
                current: Some(Box::new(self.head.clone())),
            }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list = List::new();
            let mut current = std::mem::replace(&mut self.head, Node::Nil);

            for _ in 0..n {
                match current {
                    Node::Cons(elem, next) => {
                        new_list.push(elem);
                        current = *next;
                    }
                    Node::Nil => break,
                }
            }
            new_list
        }
    }


    #[test]
    fn test_push() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Verifica che il primo elemento sia quello più recente
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Rimuovi gli elementi uno alla volta e verifica l'ordine
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None); // La lista è vuota
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None); // Lista vuota

        list.push(42);
        assert_eq!(list.peek(), Some(&42)); // Controlla il primo elemento senza rimuoverlo

        list.push(99);
        assert_eq!(list.peek(), Some(&99)); // Controlla il nuovo primo elemento
    }

    #[test]
    fn test_take() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Prendi i primi 2 elementi
        let mut new_list = list.take(2);

        // Verifica che la nuova lista contenga i primi 2 elementi
        assert_eq!(new_list.pop(), Some(2));
        assert_eq!(new_list.pop(), Some(3));
        assert_eq!(new_list.pop(), None);
    }

    struct ListIter<T> {
        current: Option<Box<Node<T>>>,
    }
    
    impl<T: Clone> Iterator for ListIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current.take() {
                Some(node) => {
                    match *node {
                        Node::Cons(elem, next) => {
                            self.current = Some(next);
                            Some(elem)
                        }
                        Node::Nil => {
                            self.current = None;
                            None
                        }
                    }
                }
                None => None,
            }
            
        }
    }

    #[test]
    fn test_iterator() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        // Verifica che l'iteratore restituisca gli elementi nell'ordine corretto
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None); // Nessun altro elemento
    }
    
}


pub mod List2 {

    #[derive(Clone)]
    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: NodeLink<T>,
    }

    // for this implementattion, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T: Clone> List<T> {
        fn new() -> Self {
            List { head: None}
        }

        fn push(&mut self, elem: T){
            let old_head = self.head.take();
            self.head = Some(Box::new(Node {
                elem, 
                next: old_head,
            }));
        }

        fn pop(&mut self) -> Option<T> {
            match self.head.take() {
                Some(node) => {
                    self.head = node.next;
                    Some(node.elem)
                }
                None => None,
            }
        }

        fn peek(&self) -> Option<&T> {
            match &self.head {
                Some(node) => Some(&node.elem),
                None => None,
            }
        }

        fn iter(&self) -> ListIter<T> {
            ListIter {
                current: self.head.clone(),
            }
        }
        

        fn take(&mut self, n: usize) -> List<T> {
            let mut new_list = List::new();
            let mut current = self.head.take();

            for _ in 0..n {
                match current {
                    Some(mut node) => {
                        new_list.push(node.elem);
                        current = node.next.take();
                    }
                    None => break,
                }
            }
            new_list
        }
    }

    #[test]
    fn test_push() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Verifica che il primo elemento sia quello più recente
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn test_pop() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Rimuovi gli elementi uno alla volta e verifica l'ordine
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None); // La lista è vuota
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None); // Lista vuota

        list.push(42);
        assert_eq!(list.peek(), Some(&42)); // Controlla il primo elemento senza rimuoverlo

        list.push(99);
        assert_eq!(list.peek(), Some(&99)); // Controlla il nuovo primo elemento
    }

    #[test]
    fn test_take() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Prendi i primi 2 elementi
        let mut new_list = list.take(2);

        // Verifica che la nuova lista contenga i primi 2 elementi
        assert_eq!(new_list.pop(), Some(2));
        assert_eq!(new_list.pop(), Some(3));
        assert_eq!(new_list.pop(), None);
    }

    struct ListIter<T> {
        current: Option<Box<Node<T>>>,
    }
    
    impl<T: Clone> Iterator for ListIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current.take() {
                Some(node) => {
                    self.current = node.next;
                    Some(node.elem.clone())
                }
                None => None,
            }
        }
    }

    #[test]
    fn test_iterator() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        // Verifica che l'iteratore restituisca gli elementi nell'ordine corretto
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None); // Nessun altro elemento
    }
}


pub mod dlist {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;
// *****
// double linked list suggestions:
// the node has both a next and a prev link

    type NodeLink<T> = Option<Rc<RefCell<DNode<T>>>>;
    type NodeBackLink<T> =  Option<Weak<RefCell<DNode<T>>>>; // weak reference to the previous node

    #[derive(Clone)]
    struct DNode<T> {
        elem: T,
        prev: NodeBackLink<T>,  // which type do we use here?
        next: NodeLink<T>, // which type do we use here?
    }

    struct DList<T> {
        head: NodeLink<T>,
        tail: NodeLink<T>
    }

    impl<T: Clone> DList<T> {
        fn new() -> Self {
            DList {
                head: None,
                tail: None,
            }
        }

        fn push_front(&mut self, elem: T) {
            let new_node = Rc::new(RefCell::new(DNode {
                elem,
                prev: None, 
                next: self.head.clone(),
            }));

            if let Some(head) = &self.head {
                (*head).borrow_mut().prev = Some(Rc::downgrade(&new_node));
            }
            else {
                self.tail = Some(new_node.clone());
            }
            self.head = Some(new_node);
        }

        fn push_back(&mut self, elem: T) {
            let new_node = Rc::new(RefCell::new(DNode {
                elem,
                prev: self.tail.as_ref().map(Rc::downgrade),
                next: None,
            }));

            if let Some(tail) = &self.tail {
                tail.borrow_mut().next = Some(new_node.clone());
            } else {
                self.head = Some(new_node.clone());
            }
            self.tail = Some(new_node);
        }

        fn pop_front(&mut self) -> Option<T> {
            match self.head.take() {
                Some(node) => {
                    let mut node = node.borrow_mut();
                    self.head = node.next.take();

                    if let Some(head) = &self.head {
                        head.borrow_mut().prev = None;
                    }
                    else {
                        self.tail = None;
                    }
                    Some(node.elem.clone())
                }
                None => None,
            }
        }

        fn pop_back(&mut self) -> Option<T> {
            match self.tail.take() {
                Some(node) => {
                    let mut node = node.borrow_mut();
                    self.tail = node.prev.as_ref().and_then(Weak::upgrade);

                    if let Some(tail) = &self.tail {
                        tail.borrow_mut().next = None;
                    } else {
                        self.head = None;
                    }
                    Some(node.elem.clone())
                }
                None => None,
            }
        }

        fn popn(&mut self, n: usize) -> Option<T> {
            let mut current = self.head.clone();
            let mut count = 0;

            while let Some(node) = current {
                let mut node = node.borrow_mut();

                if count == n {
                    let elem = node.elem.clone();

                    // Se il nodo è la testa
                    if count == 0 {
                        self.head = node.next.take();
                        if let Some(new_head) = &self.head {
                            new_head.borrow_mut().prev = None;
                        } else {
                            self.tail = None; // La lista è vuota
                        }
                    }
                    // Se il nodo è la coda
                    else if node.next.is_none() {
                        self.tail = node.prev.as_ref().and_then(Weak::upgrade);
                        if let Some(new_tail) = &self.tail {
                            new_tail.borrow_mut().next = None;
                        } else {
                            self.head = None; // La lista è vuota
                        }
                    }
                    // Nodo intermedio
                    else {
                        if let Some(prev) = node.prev.as_ref().and_then(Weak::upgrade) {
                            prev.borrow_mut().next = node.next.clone();
                        }
                        if let Some(next) = &node.next {
                            next.borrow_mut().prev = node.prev.clone();
                        }
                    }

                    return Some(elem);
                }

                current = node.next.clone();
                count += 1;
            }

            None // Indice fuori dai limiti
        }

    }


    #[test]
    fn test_push_front() {
        let mut list = DList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Verifica che il primo elemento sia quello più recente
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None); // La lista è vuota
    }

    #[test]
    fn test_push_back() {
        let mut list = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Verifica che il primo elemento sia quello meno recente
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None); // La lista è vuota
    }

    #[test]
    fn test_pop_front() {
        let mut list = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Rimuovi gli elementi uno alla volta e verifica l'ordine
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None); // La lista è vuota
    }

    #[test]
    fn test_pop_back() {
        let mut list = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Rimuovi gli elementi uno alla volta dall'ultimo e verifica l'ordine
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None); // La lista è vuota
    }

    #[test]
    fn test_mixed_operations() {
        let mut list = DList::new();
        list.push_front(1);
        list.push_back(2);
        list.push_front(3);
        list.push_back(4);

        // Verifica l'ordine degli elementi
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), None); // La lista è vuota
    }

    #[test]
    fn test_popn() {
        let mut list = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        // Rimuovi il terzo elemento (indice 2)
        assert_eq!(list.popn(2), Some(3));

        // Verifica che gli altri elementi rimangano nella lista
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), None); // La lista è vuota

        // Rimuovi la testa
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        assert_eq!(list.popn(0), Some(10)); // Rimuove la testa
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.pop_front(), None);

        // Rimuovi la coda
        list.push_back(40);
        list.push_back(50);
        list.push_back(60);
        assert_eq!(list.popn(2), Some(60)); // Rimuove la coda
        assert_eq!(list.pop_back(), Some(50));
        assert_eq!(list.pop_back(), Some(40));
        assert_eq!(list.pop_back(), None);

        // Indice fuori dai limiti
        assert_eq!(list.popn(5), None);
    }

// use Rc, since we need more than one reference to the same node. 
// You need to both strong and weak references

// For mutating the list and changing the next and prev fields we also need to be able to mutate the node, 
// therefore we can use RefCell too (as for the tree at lesson)

/*fn popn(&mut self, n : usize) -> Option<T> {
            
        } */
// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// hint for pop: you can return either a reference to the value or take the value out of the Rc, 
// but usually it is not possible to take out the value from an Rc since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  you can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc
// otherwise you can impose the COPY trait on T 

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore:
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None


}

fn main() {
    println!("Hello, world!");
}

