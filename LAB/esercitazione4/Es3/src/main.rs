
#[derive(Clone)]
struct Node {
    leftChild: Option<Box<Node>>,
    rightChild: Option<Box<Node>>,
    value: bool,
    name: String,
}

impl Node {
    pub fn new(name: String) -> Self {
        Node {
            leftChild: None,
            rightChild: None,
            value: false,
            name,
        }
    }
}

struct Albero {
    root: Option<Box<Node>>,
}

impl Albero {
    pub fn new() -> Self {
        Albero { root: None }
    }

    // Aggiungi un nodo figlio del nodo father
    pub fn add(&mut self, father: &str, node: &str) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(father.to_string())));
        }
        Self::add_recursively(self.root.as_mut(), father, node);
    }

    fn add_recursively(current: Option<&mut Box<Node>>, father: &str, node: &str) {
        if let Some(current_node) = current {
            if current_node.name == father {
                if current_node.leftChild.is_none() {
                    current_node.leftChild = Some(Box::new(Node::new(node.to_string())));
                    return;
                } else if current_node.rightChild.is_none() {
                    current_node.rightChild = Some(Box::new(Node::new(node.to_string())));
                    return;
                } else {
                    panic!("Father node already has two children");
                }
            } else {
                Self::add_recursively(current_node.leftChild.as_mut(), father, node);
                Self::add_recursively(current_node.rightChild.as_mut(), father, node);
            }
        } else {
            panic!("Father node not found");
        }
    }

    // Rimuovi un nodo e tutti i suoi eventuali rami collegati
    pub fn remove(&mut self, node: &str) {
        self.root = Self::remove_recursively(self.root.take(), node);
    }

    fn remove_recursively(current: Option<Box<Node>>, node: &str) -> Option<Box<Node>> {
        if let Some(mut current_node) = current {
            if current_node.name == node {
                return None;
            }
            current_node.leftChild = Self::remove_recursively(current_node.leftChild, node);
            current_node.rightChild = Self::remove_recursively(current_node.rightChild, node);
            Some(current_node)
        } else {
            None
        }
    }

    // Commutare l'interruttore del nodo e restituire il nuovo valore
    pub fn toggle(&mut self, node: &str) -> bool {
        if self.root.is_none() {
            panic!("Tree is empty");
        }
        if !Self::toggle_recursively(self.root.as_mut(), node) {
            panic!("Node not found");
        }
        true
    }

    fn toggle_recursively(current: Option<&mut Box<Node>>, node: &str) -> bool {
        if let Some(current_node) = current {
            if current_node.name == node {
                current_node.value = !current_node.value;
                println!("New value: {}", current_node.value);
                return true;
            } else {
                let left_result = Self::toggle_recursively(current_node.leftChild.as_mut(), node);
                if left_result {
                    return true;
                }
                let right_result = Self::toggle_recursively(current_node.rightChild.as_mut(), node);
                return right_result;
            }
        }
        false
    }

    // Restituisci se la luce è accesa o spenta
    pub fn peek(&self, node: &str) -> bool {
        if self.root.is_none() {
            panic!("Tree is empty");
        }
        Self::peek_recursively(self.root.as_ref(), node, true)
            .expect("Node not found")
    }

    fn peek_recursively(current: Option<&Box<Node>>, node: &str, all_on: bool,) -> Option<bool> {
        if let Some(current_node) = current {
            let new_all_on = all_on && current_node.value;
            if current_node.name == node {
                return Some(new_all_on);
            }
            let left_result = Self::peek_recursively(current_node.leftChild.as_ref(), node, new_all_on);
            if left_result.is_some() {
                return left_result;
            }
            let right_result = Self::peek_recursively(current_node.rightChild.as_ref(), node, new_all_on);
            return right_result;
        }
        None
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]


    #[test]
    fn test_add_node() {
        let mut tree = Albero::new();
        tree.add("root", "child1");
        tree.add("root", "child2");

        assert!(tree.root.is_some());
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.name, "root");
        assert!(root.leftChild.is_some());
        assert!(root.rightChild.is_some());
        assert_eq!(root.leftChild.as_ref().unwrap().name, "child1");
        assert_eq!(root.rightChild.as_ref().unwrap().name, "child2");
    }

    #[test]
    #[should_panic(expected = "Father node already has two children")]
    fn test_add_node_panic_on_full_children() {
        let mut tree = Albero::new();
        tree.add("root", "child1");
        tree.add("root", "child2");
        tree.add("root", "child3"); // Questo dovrebbe causare un panic
    }

    #[test]
    fn test_remove_node() {
        let mut tree = Albero::new();
        tree.add("root", "child1");
        tree.add("root", "child2");
        tree.remove("child1");

        let root = tree.root.as_ref().unwrap();
        assert!(root.leftChild.is_none());
        assert!(root.rightChild.is_some());
        assert_eq!(root.rightChild.as_ref().unwrap().name, "child2");
    }

    #[test]
    fn test_toggle_node() {
        let mut tree = Albero::new();
        tree.add("root", "child1");
        tree.add("root", "child2");

        let new_value = tree.toggle("child1");
        assert!(new_value); // Il valore dovrebbe essere commutato a true

        let root = tree.root.as_ref().unwrap();
        assert!(root.leftChild.as_ref().unwrap().value);
    }

    #[test]
    fn test_peek_node() {
        let mut tree = Albero::new();
        tree.add("root", "child1");
        tree.add("root", "child2");

        // Accendi tutte le luci
        tree.toggle("root");
        tree.toggle("child1");
        tree.toggle("child2");

        assert!(tree.peek("child1")); // Tutte le luci precedenti sono accese
        assert!(tree.peek("child2")); // Tutte le luci precedenti sono accese

        // Spegni una luce
        tree.toggle("root");
        assert!(!tree.peek("child1")); // Una luce precedente è spenta
        assert!(!tree.peek("child2")); // Una luce precedente è spenta
    }

    #[test]
    #[should_panic(expected = "Node not found")]
    fn test_toggle_nonexistent_node() {
        let mut tree = Albero::new();
        tree.add("root", "child1");
        tree.toggle("nonexistent"); // Questo dovrebbe causare un panic
    }

    #[test]
    #[should_panic(expected = "Tree is empty")]
    fn test_peek_on_empty_tree() {
        let tree = Albero::new();
        tree.peek("root"); // Questo dovrebbe causare un panic
    }

}