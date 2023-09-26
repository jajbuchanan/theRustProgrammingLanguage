// lib.rs

use std::collections::VecDeque; // VecDeque is a double-ended queue that can be efficiently
                                // appended and popped from both ends. 

struct Queue<T> { // The angled brackets indicate the use of generics. These allow you to write
                  // code that is abstracted over types. So we don't need different code for
                  // different types. The 'Queue' struct here is a generic struct that can hold
                  // items of any type 'T'. T is a convention, and often used for Type. We might
                  // use E for Element or Error, K and V for Key and Value. 
                  // So we're defining a generic struct named 'Queue' with a single type parameter
                  // 'T'
    pub items: VecDeque<T>, // The 'pub' here means that the items field in the Queue struct is
                            // public, meaning it can be accessed from outside the struct. 
                            // The line declares a new generic struct named Queue, and inside the
                            // struct there is a single field named 'items' of type VecDeque<T>. 
}

impl<T> Queue<T> { // What does impl do here? -> The impl keyword hear means thtis is an
                   // implementation block for the generic `Queue<T>`. The <T> indicates it's a
                   // generic implementation, so the methods we define within this block can work
                   // with Queues of any type T
    pub fn new() -> Queue<T> { // This declares a public function named 'new' that takes no
                               // parameters and returns an instance of Queue<T> 
        Queue {
            items: VecDeque::new(), // Inside the function, we are creating a new 'Queue' instance
                                    // where the 'items' field is initialized with an empty
                                    // VecDeque (using VecDeque::new())
        }
    }

    pub fn enqueue(&mut self, v: T) { // This is a method named 'enqueue'. 
                                      // pub fn enqueue(&mut self, v: T) declares a public method
                                      // named 'enqueue that takes a mutable reference to the
                                      // current Queue instance (&mut self) and a value v of type T
        self.items.push_back(v) // This appends the value 'v' to the end of the 'VecDeque' (i.e.
                                // enqueues the item)
    } 

    pub fn dequeue(&mut self) -> T { // This declares a public method named dequeue that takes a
                                     // mutable reference to the current Queue instanec and returns
                                     // a value of type T
        self.items
            .pop_front() // This tries to pop (dequeue) an item from the front of the VecDequeue
            .expect("Cannot dequeue from empty queue.") // If pop_front returns 'None' because the
                                                        // VecDeque is empty, the program will
                                                        // panic with the provided error message
    }

    pub fn is_empty(&self) -> bool { // This declares a public method named is_empty that takes an
                                     // immutable reference to the current Queue instance (&self)
                                     // and returns a bool
        self.items.len() == 0 // This checks if the length of the VecDeque is zero. If it is,
                              // the method returns 'true', indivating that the queue is empty;
                              // otherwise it returns 'false'
    }
}
