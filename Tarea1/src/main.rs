mod utils;
use std::collections::HashMap;
use utils::{queue::Queue, stack::Stack};

// Correr cargo run para ver el resultado de la funcion main
// Alternativamente compilar el archivo main con rustc src/main.rs y correr el ejecutable

// Para correr las pruebas usar cargo test
// Alternativamente compilar el archivo con rustc --test src/main.test y correr el ejecutable

fn main() {
    let placeholder_words = ["Holaa", "Mundo", "Rust", "Compis"];
    let mut stack = Stack::new();

    for word in placeholder_words {
        stack.push(word);
    }

    let top = stack.top();
    println!("Top element in stack: {:?}", top);
    stack.pop();
    println!("Top element after in stack a pop: {:?}", stack.top());

    let mut queue = Queue::new();
    for word in placeholder_words {
        queue.push(word);
    }

    println!("Top element in queue: {:?}", queue.peek());
    queue.pop();
    println!("Top element after in queue a pop: {:?}", queue.peek());

    let mut hm = HashMap::new();
    match queue.peek() {
        Some(word) => {
            hm.insert(word, 2);
        }
        None => {
            println!("{}", "Couldnt find word");
        }
    }
    match queue.peek() {
        Some(word) => match hm.get(word) {
            Some(found_word_value) => {
                assert_eq!(found_word_value, &2);
                println!("Found word: {} with value: {:?}", word, found_word_value);
            }
            None => {
                println!("Couldn't find a value for the word {}", word)
            }
        },
        None => {
            println!("{}", "Couldnt find word");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());

        stack.push("Test");
        assert_eq!(stack.top(), Some(&"Test"));

        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_queue_operations() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());

        queue.push("Test");
        assert_eq!(queue.peek(), Some(&"Test"));

        queue.pop();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_hashmap_with_queue() {
        let mut queue = Queue::new();
        queue.push("TestWord");

        let mut hm = HashMap::new();
        if let Some(word) = queue.peek() {
            hm.insert(word, 2);

            let value = hm.get(word);
            assert_eq!(value, Some(&2));
        } else {
            panic!("Queue peek returned None when it should have a value");
        }
    }
}
