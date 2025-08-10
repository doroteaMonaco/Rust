## ESERCIZIO 1
VAL -> 
    - STACK -> 1B (tag) + sizeof(T) + padding
NEXT -> 
    - STACK -> 8B (puntatore)
    - HEAP -> Contiene la Linked List, contenuta dentro il box puntato da next.

(nel caso del puntatore dentro l'option il tag non si mette)
Il fine lista si può indicare con un None.

## ESERCIZIO 2
In un programma multi-thread, le race conditions si possono evitare usando dei scostrutti specifici a seconda del tipo di risorsa. 
Nel caso di uno scalare si definisce una tupla composta da un Mutex che contiene lo scalare e dalla Condavr per gestire l'attesa e il risveglio. Inoltre, Rust mette a disposizione anche i tipi Atomic, di cui esistono diversi tipi, che rendono le istruzioni indivisibili e si appoggiano ad istruzioni di tipo fence o barrier.
Arc non è necessarrio in quanto i tipi primitivi implementano Copy e quindi copiano il dato nel momento della condivisione. Per struttire più complesse, come struct, uso Arc<Mutex> in coppia con una condvar poichè Arc rende la condivisione thread-safe.
L'uso del Mutex, permette di applicare il paradigma RAII: ovvero ua volta che esce dal suo scope, rilascia automaticamente le risorse.

```rust
{
    //scalare
    let num = Arc::new((Mutex::new(0)), Condvar::new());
    let n = Arc::new(AtomicUsize::new(0));
    //struct
    let s = Arc::new((Mutex::new(s), Condvar::new()));
}
```




