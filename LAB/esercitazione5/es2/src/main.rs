use std::cell::{RefCell, Ref, RefMut};
use std::rc::{Rc, Weak};

#[derive(Clone)]
enum FSItem { 
    Directory(Directory), // Directory contiene nome, i figli, eventuali metadati, il padre 
    File(File), // File contiene il nome, eventuali metadati (es dimensione, owner, ecc), il padre 
    SymLink(Link) // Il link simbolico contiene il Path a cui punta e il padre 
}

#[derive(Clone)]
enum MetaData {
    Size(u64), // dimensione in byte
    Owner(String), // nome del proprietario
    Permissions(String), // permessi di accesso
}

#[derive (Clone)]
struct Directory {
    name: String,
    children: RefCell<Vec<FSItem>>, // Use RefCell<Vec<FSItem>> for interior mutability
    metadati: Option<Vec<MetaData>>,
    father: Option<RefCell<Weak<Directory>>>, // il padre è una Weak per evitare cicli di riferimento
}

#[derive (Clone)]
struct File {
    name: String,
    metadati: Option<Vec<MetaData>>,
    father: Option<RefCell<Weak<Directory>>>, // il padre è una Weak per evitare cicli di riferimento
}

#[derive (Clone)]
struct Link {
    path: String,
    father: Option<RefCell<Weak<File>>>,
}

struct FileSystem {
    root: RefCell<Rc<Directory>>, // la root del file system
    current: RefCell<Rc<Directory>>, // la directory corrente
}

struct Result {
    success: bool,
    message: String,
}

impl FileSystem { 
    // crea un nuovo FS vuoto 
    pub fn new() -> Self{
        let root = Directory {
            name: String::from("/"),
            children: RefCell::new(Vec::new()),
            metadati: None, 
            father: None,
        };
        let current = root.clone();
        FileSystem {
            root: RefCell::new(Rc::new(root)),
            current: RefCell::new(Rc::new(current)),
        }
    }     

    // crea un nuovo FS replicando la struttura su disco 
    pub fn from_disk() -> Self{
        let root = Directory {
            name: String::from("/"),
            children: RefCell::new(Vec::new()),
            metadati: None, 
            father: None,
        };
        let current = root.clone();
        FileSystem {
            root: RefCell::new(Rc::new(root)),
            current: RefCell::new(Rc::new(current)),
        }
    }

    // Cambia la directory corrente
    pub fn change_dir(&mut self, path: String) -> Result {
        let mut current = self.current.borrow_mut();
        let mut new_dir = self.root.borrow().clone();

        // Se il path è assoluto, inizia dalla root
        if path.starts_with("/") {
            new_dir = self.root.borrow().clone();
        } else {
            new_dir = current.clone();
        }

        // Split the path into components
        let components: Vec<&str> = path.split('/').collect();

        for component in components {
            if component == ".." {
                // Torna alla directory padre
                if let Some(father) = new_dir.father.as_ref().and_then(|f| f.borrow().upgrade()) {
                    new_dir = father;
                } else {
                    return Result {
                        success: false,
                        message: String::from("Already at the root directory"),
                    };
                }
            } else if component != "." && !component.is_empty() {
                // Naviga nei figli
                let children = new_dir.children.borrow();
                if let Some(next_dir) = children.iter().find_map(|child| {
                    if let FSItem::Directory(dir) = child {
                        if dir.name == component {
                            return Some(dir.clone());
                        }
                    }
                    None
                }) {
                    new_dir = next_dir;
                } else {
                    return Result {
                        success: false,
                        message: format!("Directory '{}' not found", component),
                    };
                }
            }
        }

        *current = new_dir;
        Result {
            success: true,
            message: String::from("Directory changed successfully"),
        }
    }

    // Crea una directory
    pub fn make_dir(&self, path: String, name: String) -> Result {
        let mut current = self.current.borrow_mut();
        let mut new_dir = self.root.borrow().clone();

        // Se il path è assoluto, inizia dalla root
        if path.starts_with("/") {
            new_dir = self.root.borrow().clone();
        } else {
            new_dir = current.clone();
        }

        // Split the path into components
        let components: Vec<&str> = path.split('/').collect();

        for component in components {
            if component == ".." {
                if let Some(father) = new_dir.father.as_ref().and_then(|f| f.borrow().upgrade()) {
                    new_dir = father;
                }
            } else if component != "." && !component.is_empty() {
                let next_dir = {
                    let children = new_dir.children.borrow();
                    children.iter().find_map(|child| {
                        if let FSItem::Directory(dir) = child {
                            if dir.name == component {
                                return Some(dir.clone());
                            }
                        }
                        None
                    })
                };
                if let Some(next_dir) = next_dir {
                    new_dir = next_dir;
                }
            }
        }

        // Crea la nuova directory
        let new_directory = Directory {
            name,
            children: RefCell::new(Vec::new()),
            metadati: None,
            father: Some(RefCell::new(Rc::downgrade(&new_dir))),
        };

        new_dir.children.borrow_mut().push(FSItem::Directory(new_directory));

        Result {
            success: true,
            message: String::from("Directory created successfully"),
        }
    }

    // Crea un file
    pub fn make_file(&self, path: String, name: String) -> Result {
        let mut current = self.current.borrow_mut();
        let mut new_dir = self.root.borrow().clone();

        // Se il path è assoluto, inizia dalla root
        if path.starts_with("/") {
            new_dir = self.root.borrow().clone();
        } else {
            new_dir = current.clone();
        }

        // Split the path into components
        let components: Vec<&str> = path.split('/').collect();

        for component in components {
            if component == ".." {
                if let Some(father) = new_dir.father.as_ref().and_then(|f| f.borrow().upgrade()) {
                    new_dir = father;
                }
            } else if component != "." && !component.is_empty() {
                let next_dir = {
                    let children = new_dir.children.borrow();
                    children.iter().find_map(|child| {
                        if let FSItem::Directory(dir) = child {
                            if dir.name == component {
                                return Some(dir.clone());
                            }
                        }
                        None
                    })
                };
                if let Some(next_dir) = next_dir {
                    new_dir = next_dir;
                }
            }
        }

        // Crea il nuovo file
        let new_file = File {
            name,
            metadati: None,
            father: Some(RefCell::new(Rc::downgrade(&new_dir))),
        };

        new_dir.children.borrow_mut().push(FSItem::File(new_file));

        Result {
            success: true,
            message: String::from("File created successfully"),
        }
    }

    // Rinomina un file o una directory
    pub fn rename(&self, path: String, new_name: String) -> Result {
        let mut current = self.current.borrow_mut();
        let mut new_dir = self.root.borrow().clone();

        // Se il path è assoluto, inizia dalla root
        if path.starts_with("/") {
            new_dir = self.root.borrow().clone();
        } else {
            new_dir = current.clone();
        }

        // Split the path into components
        let components: Vec<&str> = path.split('/').collect();

        for component in components.iter().take(components.len() - 1) {
            if component == ".." {
                if let Some(father) = new_dir.father.as_ref().and_then(|f| f.borrow().upgrade()) {
                    new_dir = father;
                }
            } else if component != "." && !component.is_empty() {
                let children = new_dir.children.borrow();
                if let Some(next_dir) = children.iter().find_map(|child| {
                    if let FSItem::Directory(dir) = child {
                        if dir.name == *component {
                            return Some(dir.clone());
                        }
                    }
                    None
                }) {
                    new_dir = next_dir;
                }
            }
        }

        // Rinomina l'elemento
        let target_name = components.last().unwrap();
        let mut children = new_dir.children.borrow_mut();
        if let Some(child) = children.iter_mut().find(|child| match child {
            FSItem::Directory(dir) => dir.name == *target_name,
            FSItem::File(file) => file.name == *target_name,
            _ => false,
        }) {
            match child {
                FSItem::Directory(dir) => dir.name = new_name.clone(),
                FSItem::File(file) => file.name = new_name.clone(),
                _ => {}
            }
            Result {
                success: true,
                message: String::from("Element renamed successfully"),
            }
        } else {
            Result {
                success: false,
                message: format!("Element '{}' not found", target_name),
            }
        }
    }

    // Cancella un file o una directory
    pub fn delete(&self, path: String) -> Result {
        let mut current = self.current.borrow_mut();
        let mut new_dir = self.root.borrow().clone();

        // Se il path è assoluto, inizia dalla root
        if path.starts_with("/") {
            new_dir = self.root.borrow().clone();
        } else {
            new_dir = current.clone();
        }

        // Split the path into components
        let components: Vec<&str> = path.split('/').collect();

        for component in components.iter().take(components.len() - 1) {
            if component == ".." {
                if let Some(father) = new_dir.father.as_ref().and_then(|f| f.borrow().upgrade()) {
                    new_dir = father;
                }
            } else if component != "." && !component.is_empty() {
                let children = new_dir.children.borrow();
                if let Some(next_dir) = children.iter().find_map(|child| {
                    if let FSItem::Directory(dir) = child {
                        if dir.name == *component {
                            return Some(dir.clone());
                        }
                    }
                    None
                }) {
                    new_dir = next_dir;
                }
            }
        }

        // Cancella l'elemento
        let target_name = components.last().unwrap();
        let mut children = new_dir.children.borrow_mut();
        if let Some(pos) = children.iter().position(|child| match child {
            FSItem::Directory(dir) => dir.name == *target_name,
            FSItem::File(file) => file.name == *target_name,
            _ => false,
        }) {
            let child = children.remove(pos);
            if let FSItem::Directory(dir) = child {
                Self::delete_directory_contents(&dir);
            }
            Result {
                success: true,
                message: String::from("Element deleted successfully"),
            }
        } else {
            Result {
                success: false,
                message: format!("Element '{}' not found", target_name),
            }
        }
    }

    // Funzione di supporto per eliminare ricorsivamente il contenuto di una directory
    fn delete_directory_contents(dir: &Directory) {
        let mut children = dir.children.borrow_mut();
        children.clear(); // Rimuove tutti i figli della directory
    }

    // cerca l’elemento indicato dal path e restituisci un riferimento 
    pub fn find(&self, path: String) -> Result {
        let mut current = self.current.borrow_mut();
        let mut root = self.root.borrow_mut();
        let mut new_dir = root.clone();

        // Se il path è assoluto, inizia dalla root
        if path.starts_with("/") {
            new_dir = root.clone();
        } else {
            new_dir = current.clone();
        }

        // Split the path into components
        let components: Vec<&str> = path.split('/').collect();

        for component in components {
            if *component == ".." {
                // Usa una variabile temporanea per evitare conflitti di borrowing
                let father = new_dir.father.as_ref().and_then(|f| f.borrow().upgrade());
                if let Some(father) = father {
                    new_dir = father;
                }
            } else if *component != "." && !component.is_empty() {
                // Usa una variabile temporanea per evitare conflitti di borrowing
                let next_dir = {
                    let children = new_dir.children.borrow();
                    children.iter().find_map(|child| {
                        if let FSItem::Directory(dir) = child {
                            if dir.name == *component {
                                return Some(dir.clone());
                            }
                        }
                        None
                    })
                };
                if let Some(next_dir) = next_dir {
                    new_dir = next_dir.into();
                }
            }
        }

        Result {
            success: true,
            message: String::from("Element found successfully"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fs = FileSystem::new();

        // Verifica che la root sia inizializzata correttamente
        assert_eq!(fs.root.borrow().name, "/");
        assert!(fs.root.borrow().children.borrow().is_empty());
        assert!(fs.root.borrow().father.is_none());

        // Verifica che la directory corrente sia la root
        assert_eq!(fs.current.borrow().name, "/");
    }

    #[test]
    fn test_change_dir() {
        let mut fs = FileSystem::new();

        // Crea alcune directory per il test
        fs.make_dir(String::from("/"), String::from("dir1"));
        fs.make_dir(String::from("/dir1"), String::from("dir2"));

        // Cambia directory in una sottodirectory
        let result = fs.change_dir(String::from("dir1"));
        assert!(result.success);
        assert_eq!(fs.current.borrow().name, "dir1");

        // Cambia directory in una sottodirectory di dir1
        let result = fs.change_dir(String::from("dir2"));
        assert!(result.success);
        assert_eq!(fs.current.borrow().name, "dir2");

        // Torna alla directory padre
        let result = fs.change_dir(String::from(".."));
        assert!(result.success);
        assert_eq!(fs.current.borrow().name, "dir1");

        // Torna alla root
        let result = fs.change_dir(String::from("/"));
        assert!(result.success);
        assert_eq!(fs.current.borrow().name, "/");

        // Cambia directory con un percorso non valido
        let result = fs.change_dir(String::from("nonexistent"));
        assert!(!result.success);
    }

    #[test]
    fn test_make_dir() {
        let mut fs = FileSystem::new();

        // Crea una directory nella root
        let result = fs.make_dir(String::from("/"), String::from("dir1"));
        assert!(result.success);

        // Verifica che la directory sia stata creata
        let root_ref = fs.root.borrow();
        let root_children = root_ref.children.borrow();
        assert_eq!(root_children.len(), 1);
        if let FSItem::Directory(dir) = &root_children[0] {
            assert_eq!(dir.name, "dir1");
            assert!(dir.children.borrow().is_empty());
        } else {
            panic!("Expected a directory");
        }
    }

    #[test]
    fn test_make_file() {
        let mut fs = FileSystem::new();

        // Crea un file nella root
        let result = fs.make_file(String::from("/"), String::from("file1.txt"));
        assert!(result.success);

        // Verifica che il file sia stato creato
        let root_ref = fs.root.borrow();
        let root_children = root_ref.children.borrow();
        assert_eq!(root_children.len(), 1);
        if let FSItem::File(file) = &root_children[0] {
            assert_eq!(file.name, "file1.txt");
            assert!(file.metadati.is_none());
        } else {
            panic!("Expected a file");
        }
    }

    #[test]
    fn test_rename() {
        let mut fs = FileSystem::new();

        // Crea una directory e un file
        fs.make_dir(String::from("/"), String::from("dir1"));
        fs.make_file(String::from("/"), String::from("file1.txt"));

        // Rinomina la directory
        let result = fs.rename(String::from("/dir1"), String::from("dir2"));
        assert!(result.success);

        // Verifica che la directory sia stata rinominata
        let root_ref = fs.root.borrow();
        let root_children = root_ref.children.borrow();
        assert!(root_children.iter().any(|child| match child {
            FSItem::Directory(dir) => dir.name == "dir2",
            _ => false,
        }));

        // Rinomina il file
        let result = fs.rename(String::from("/file1.txt"), String::from("file2.txt"));
        assert!(result.success);

        // Verifica che il file sia stato rinominato
        assert!(root_children.iter().any(|child| match child {
            FSItem::File(file) => file.name == "file2.txt",
            _ => false,
        }));
    }

    #[test]
    fn test_delete() {
        let mut fs = FileSystem::new();

        // Crea una directory e un file
        fs.make_dir(String::from("/"), String::from("dir1"));
        fs.make_file(String::from("/"), String::from("file1.txt"));

        // Elimina la directory
        let result = fs.delete(String::from("/dir1"));
        assert!(result.success);

        // Verifica che la directory sia stata eliminata
        let root_ref = fs.root.borrow();
        let root_children = root_ref.children.borrow();
        assert!(!root_children.iter().any(|child| match child {
            FSItem::Directory(dir) => dir.name == "dir1",
            _ => false,
        }));

        // Elimina il file
        let result = fs.delete(String::from("/file1.txt"));
        assert!(result.success);

        // Verifica che il file sia stato eliminato
        assert!(!root_children.iter().any(|child| match child {
            FSItem::File(file) => file.name == "file1.txt",
            _ => false,
        }));
    }
}

fn main() {
    println!("Hello, world!");
}
