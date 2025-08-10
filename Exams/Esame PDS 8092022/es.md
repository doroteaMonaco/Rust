## ESERCIZIO 1

# Struttura 1

**rc
STACK -> 8B
HEAP -> 8B + 8B + 8B = 24B

in rc i contatori sono di tipo Cell<usize> perchè non condivisibili
in arc no, sono thread-safe

**rc2
STACK -> 8B
HEAP -> dati già contenuti da rc

**wk
STACK -> 8B
HEAP -> punta agli stessi di rc (downgrade)

# Struttura 2

**vector 
STACK -> 8B + 8B + 8B = 24B
HEAP ->  8B*8 = 64B

**vslice

STACK -> 8B + 8B = 16B
HEAP -> già occupato dal vettore

## ESERCIZIO 2

Il programma crea un writer e due reader tramite thread. Il writer, aspetta per un secondo ad ogni iterazione e scrive per 5 volte su un vettore data e notifica la scrittura ai reader.
I readers lavorano in loop: aspettano tramite una wait la notify del writer e quando la ricevono leggono il valore.
Il problema principale è che nel momento in cui il writer finirà le sue 5 scritture, i reader rimarrano in attesa infinita. Basta inserire un flag che controlla quando il writer finisce di scrivere e rompere, solo i quel momento, il loop dei reader.

## ESERCIZIO 3

Il programma chiama un vettore mutabile e fa una prima push (60). Poi definisce la closure process_data e chiama una push (40). Dopo chiama process_data. Dentro il suo scope data viene però consumato, quindi il problema si pone nel momento in cui faccio l'ultima push (30). 
Soluzione -> clono data e lo passo all'interno della closure.

## ESERCIZIO 4

Le principali aree di memoria di un eseguibile sono:
- segmento di codice:
- segmento di costanti
- segmento di variabili globali 
- heap
- stack

# Esempio

```rust
{
    const: i32 = 4; //costante e variabile globale
    let mut vec: Vec<i32> = Vec::with_capacity(8); 
    //puntatore + length + capacity nello stack
    //8 * sizeof(i32) nello heap
}
```

## ESERCIZIO 5

La differenza tra Cell e RefCell:
- Cell: effettua una copia mutabile anche se il contenitore di partenza non lo era. Richiede però che il tipo T implementi il tratto Copy e usa i metodi set e get.
- RefCell: permette il mutuo accesso ad un valore con i metodi borrow e borrow_mut.

In termini di memoria:
- Cell contiene esattamente il dato T, ovvero quello che copia
- RefCell ha un puntatore a T e il campo borrow.

## ESERCIZIO 5

In un programma che implementa condition variable è possibile che alcune notifiche vengano perse, ad esempio quando un thread fa la notify_all() prima che un thread si metta in wait.

```rust
{
 let mut mutex = mutex.lock().unwrap();
 mutex = cv.wait(mutex).unwrap();
}
{
 cv.notify_one();
}
```

Possibili soluzioni ->
```rust
{
    let mut mutex= mutex.lock().unwrap();
    while (condition(*mutex)) {
        mutex = cv.wait(mutex).unwrap();
    }
}
//t1
{
 let mut mutex = mutex.lock().unwrap();
 mutex = cv.wait_while(mutex, |m| condition(m) ).unwrap();
}
```