struct Node {
    father: Option<Box<Node>>,
    leftChild: Option<Box<Node>>,
    rightChild: Option<Box<Node>>,
    value: bool,
    name: String,
}

impl Node {
    pub fn new(name: String) -> Self {
        Node {
            father: None,
            leftChild: None,
            rightChild: None,
            value: false,
            name,
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            father: self.father.clone(),
            leftChild: self.leftChild.clone(),
            rightChild: self.rightChild.clone(),
            value: self.value,
            name: self.name.clone(),
        }
    }
}

struct Albero {
    root: Option<Box<Node>>,
}
impl Albero {
    pub fn new() -> Self {
        Albero {
            root: None,
        }
    }
    // nota: aggiustare mutabilità dove necessario gestire errori in caso
    // di collisioni, valori mancanti
    // aggiungi un nodo figlio del nodo father
    pub fn add(&mut self, father: &str, node: &str) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(father.to_string())));
        }
        Self::add_recursively(self.root.as_mut(), father, node);
    }

    pub fn add_recursively(current: Option<&mut Box<Node>>, father: &str, node: &str) {
        if let Some(current_node) = current {
            if current_node.name == father {
                if current_node.leftChild.is_none() {
                    let mut new_node = Box::new(Node::new(node.to_string()));
                    new_node.father = Some(Box::clone(current_node)); // Imposta il padre
                    current_node.leftChild = Some(new_node);
                    return;
                } else if current_node.rightChild.is_none() {
                    let mut new_node = Box::new(Node::new(node.to_string()));
                    new_node.father = Some(Box::clone(current_node)); // Imposta il padre
                    current_node.rightChild = Some(new_node);
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
    // togli un nodo e tutti gli eventuali rami collegati
    pub fn remove(&mut self, node: &str) {
        self.root = Self::remove_recursively(self.root.take(), node);
    }

    fn remove_recursively(current: Option<Box<Node>>, node: &str) -> Option<Box<Node>> {
        if let Some(mut current_node) = current {
            if current_node.name == node {
                return None; // Rimuovi il nodo corrente
            }

            // Ricorsivamente esplora i figli sinistro e destro
            current_node.leftChild = Self::remove_recursively(current_node.leftChild, node);
            current_node.rightChild = Self::remove_recursively(current_node.rightChild, node);

            Some(current_node) // Restituisci il nodo aggiornato
        } else {
            None // Nodo non trovato
        }
    }
    // commuta l’interruttore del nodo (che può essere on off) e restituisci il
    //nuovo valore
    pub fn toggle(&mut self, node: &str) -> bool {
        if self.root.is_none() {
            panic!("Tree is empty");
        }
        if !Self::toggle_recursively(self.root.as_mut().unwrap(), node) {
            panic!("Node not found");
        }
        true // Restituisci il nuovo valore del nodo
    }

    fn toggle_recursively(current: &mut Node, node: &str) -> bool {
        // Se il nodo corrente è quello cercato, commuta il valore e restituiscilo
        if current.name == node {
            current.value = !current.value;
            return current.value;
        }

        // Cerca ricorsivamente nei figli sinistro e destro
        if let Some(left) = current.leftChild.as_mut() {
            if Self::toggle_recursively(left, node) {
                return true;
            }
        }
        if let Some(right) = current.rightChild.as_mut() {
            if Self::toggle_recursively(right, node) {
                return true;
            }
        }

        // Se il nodo non viene trovato in nessuno dei rami, restituisci false
        false
    }
    // restituisci se la luce è accesa e spenta: Ricorda che è accessa se tutte le luci dei nodi precedenti sono accese
    // e spenta se almeno una delle luci dei nodi precedenti è spenta
    pub fn peek(&self, node: &str) -> bool {
        if self.root.is_none() {
            panic!("Tree is empty");
        }

        // Trova il nodo specificato e verifica lo stato dei suoi antenati
        Self::peek_recursively(self.root.as_ref().unwrap(), node)
    }

    fn peek_recursively(current: &Node, node: &str) -> bool {
        // Se il nodo corrente è quello cercato, verifica il suo stato e risali
        if current.name == node {
            return Self::check_ancestors(current);
        }

        // Cerca ricorsivamente nei figli sinistro e destro
        if let Some(left) = current.leftChild.as_ref() {
            if Self::peek_recursively(left, node) {
                return true;
            }
        }
        if let Some(right) = current.rightChild.as_ref() {
            if Self::peek_recursively(right, node) {
                return true;
            }
        }

        false // Nodo non trovato
    }

    fn check_ancestors(node: &Node) -> bool {
        // Se il nodo corrente ha la luce spenta, restituisci false
        if !node.value {
            return false;
        }

        // Se il nodo corrente non ha un padre, siamo arrivati alla radice
        if let Some(father) = node.father.as_deref() {
            // Verifica ricorsivamente il padre
            return Self::check_ancestors(father);
        }

        true // Tutti i padri hanno la luce accesa
    }
}  


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(new_value); // Il valore dovrebbe essere commutato a `true`

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
