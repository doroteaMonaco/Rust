
### ESERCIZIO 1

Gli smart pointer, come Rc sono caratterizzati da contatori di riferimenti forti e deboli. Nel momento in cui il contatore va a 0 la memoria viene deallocata. Due Rc non possono puntarsi a vicenda poichè i due contatori continuerebbero a incrementarsi a vicenda, non raggiungendo mai lo zero. Si genererebbe quindi Memory Leakege. Per questo motivo, vengono introdotti i Weak.

### ESERCIZIO 2

```rust
struct Data {
    Element: AsVector,
    next: Rc<Data>
}

enum AsVector {
    AsVector(Box::<Rc<i32>>), //Rc in HEAP
    None
}
```

## AsVector

Stack -> tag (1B) + T (8B (Box)) + padding (allineamento) = 16B

## Data

Stack -> 16B (AsVector) + 8B (Rc) = 24B
Heap (caso peggiore) -> 8B (contStrong) + 8B (contWeak) + 4B (i32) = 20B + padding (allineamento) = 24B + Data = 24B + 24B = 48B

##

numero complessivo = 24B + 48B = 72B

### ESERCIZIO 3

I tratti principali della concorrenzza sono:
- Send
- Sync

Send permette di spostare un tipo tra thread, Sync permette di creare riferimenti immutabili di un tipo condivisibili da tanti thread.

La mutabilità è offerta da Mutex ma non è condiviso da più thread, se vogliamo ciò usiamo RwLock che permette accessi multipli in lettura e unico in scrittura. 
Tutto dentro Arc per essere thread-safe.

