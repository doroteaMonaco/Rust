## ESERCIZIO 1
Gli smart pointers sono puntatori intelligenti che mettono a disposizione tecniche efficace perla gestione dei riferimenti: come ad esempio le dipendenze cicliche di Rc e Weak mediante puntatori.
Il possesso di uno smart pointer è la capcità di un puntatore di possedere il valore a cui punta, tranne per Rc che permette la creazione di molteplici riferimenti e quindi più owner. Quando il puntatore verrà deallocato verrà deallocata anche la variabile puntata, contenuta nello heap, e quindi si evita memoty leakage.

#### Codice 

1) Dichiaro il valore i come mutabile e gli assegno il valore 10 -> allocato sullo stack.
2) Creo uno smart pointer di tipo Box che punterà i -> sullo stack alloco il nuovo puntatore che avrà sullo heap il valore 10, ma non è lo stesso di i (i implementa il tratto Copy).
3) Creo un secondo smart pointer, mutabile, che punta al valore contenuto nel primo smart pointer -> sullo stack avrò il secondo puntatore che punterà al valore 10 sullo heap, ma non è lo stesso valore di bi1 (copia).
4) Cambio il valore contenuto del secondo smart pointer assegnandoli 20 -> sullo heap aggiorno il valore puntato a 20.
5) Cambio il valore di i assegnandoli il valore puntato dal secondo smart pointer -> cambio i nello stack.
Per cui la stampa finale di i, bi1, bi2 sarà 20, 10, 20.

## ESERCIZIO 2

Ownership System (sistema di proprietà)
In Rust, ogni valore ha un proprietario e solo uno alla volta. Quando il proprietario esce dallo scope, il valore viene automaticamente deallocato.

Regole fondamentali:
Ogni valore ha un proprietario.

Un solo proprietario alla volta.

Quando il proprietario esce dallo scope → drop automatico (deallocazione).

➡️ Effetto: si evitano memory leaks e deallocazioni doppie.

 Borrowing e Lifetimes
Invece di copiare o spostare un valore, Rust permette di "prenderlo in prestito" (&T per riferimento immutabile, &mut T per mutabile).

Il compilatore verifica che:

Non ci siano due riferimenti mutabili contemporaneamente.

Non ci siano riferimenti attivi dopo che il dato è stato liberato.

➡️ Effetto: si evitano dangling pointers (puntatori pendenti) e data race.

Rust non ha null per default. Al suo posto usa: Option<T>

➡️ Effetto: nessun null pointer dereference, perché il compilatore ti obbliga a gestire i casi None.

Molti linguaggi scoprono errori di memoria a runtime (es. segfault), ma Rust li previene a compile time, grazie al borrow checker.

➡️ Effetto: si evita accedere a memoria non più valida (dangling reference).

Sicurezza e Performance: niente Garbage Collector
Rust non usa un garbage collector (come Java o Go).

Tutta la memoria è gestita in modo deterministico e prevedibile.

Il codice prodotto è spesso comparabile in prestazioni a C/C++, ma con garanzie di sicurezza molto superiori.

✅ Wild Pointer
Non è inizializzato (valore casuale)

Può puntare a qualsiasi indirizzo

Scrivere/leggere da esso = comportamento indefinito

✅ Dangling Pointer
Era valido, ma non lo è più (l’oggetto a cui puntava è stato distrutto o deallocato)

L'accesso sembra corretto ma punta a memoria non più sicura

## ESERCIZIO 3

In Rust la gestione dell'errore viene effettuata mediante due costrutti:
- Result<R,E> -> Ritorna Ok(R) se è corretto o un errore.
- Option<V> -> Ritorna un Some(V) se il valore esiste o un None se è NULL.

Sia per Result che per Option uso unwrap per ottenre il valore dentro il Some/Ok.