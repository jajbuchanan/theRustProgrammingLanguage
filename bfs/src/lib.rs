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
type Vertex = Vec<u32>;
type Graph = Vec<Vertex>;


fn bfs(graph: Graph, start_node: u32, end_node: u32) -> Option<Vec<Option<u32>>> {
    let mut queue = Queue::new();
    queue.enqueue(start_node);

    let mut visited_vertices = vec![false; graph.len()];
    visited_vertices[0] = true;

    let mut prev: Vec<Option<u32>> = vec![None; graph.len()];

    'outer: while !queue.is_empty() {
        let current_node = queue.dequeue();
        for v in graph[current_node as usize].iter() {
            if *v == end_node {
                prev[*v as usize] = Some(current_node);
                break 'outer;
            }

            if !visited_vertices[*v as usize] {
                queue.enqueue(*v);
                visited_vertices[*v as usize] = true;
                prev[*v as usize] = Some(current_node);
            }
        }
    }

    let mut path = Vec::new();
    let mut at = Some(end_node);
    while at != None {
        path.push(at);
        at = prev[at.unwrap_or(0) as usize];
    }

    path.reverse();

    return match path[0] {
        Some(x) if x == start_node => Some(path),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_path() {
        let G: Graph = vec![
            vec![1, 2],
            vec![0, 3, 4, 1],
            vec![0, 4], 
            vec![1, 4, 5], 
            vec![1, 2, 3, 5],
            vec![3, 4, 6],
            vec![7, 5],
            vec![6],
        ];

        assert_eq!(
            bfs(G, 0, 7).unwrap(),
            vec![Some(0), Some(1), Some(3), Some(5), Some(6), Some(7)]
        )
    }

    #[test]
    fn no_existing_path() {
        let G: Graph = vec![
            vec![1, 2, 5],
            vec![0, 1, 3, 4],
            vec![0, 3],
            vec![1, 4, 5, 2],
            vec![1, 3, 5],
            vec![0, 3, 4, 1],
            vec![7],
            vec![6],
        ];

        assert_eq!(bfs(G, 0, 7), None)
    }
}
