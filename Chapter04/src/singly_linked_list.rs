use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone,Debug)]
struct Node {
    value: String,
    next: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

#[derive(Clone,Debug)]
pub struct TransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }

    pub fn append(&mut self, value: String) { 
        let new = Node::new(value);
        eprintln!(" = {:?}", new);
        match self.tail.take() { 
            Some(old) => old.borrow_mut().next = Some(new.clone()), 
            None => self.head = Some(new.clone())
        };    
        self.length += 1;
        self.tail = Some(new);
        eprintln!(" = {:?}", self.tail);

    }

    pub fn pop(&mut self) -> Option<String> {
        eprintln!(" = {:?}", self.head);

		//TODO: fix error
		let mut copy_head = self;
		match self.head.take() {
			Some(old) => {
				self.head = old.borrow_mut().next;
			},
			None => {
				self.head = self.tail;
			}
		}
		self.length -=1;

        /* self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    } */

 /*    pub fn popEnd<>(& mut self) -> Option<String> {

    } */

 
}
