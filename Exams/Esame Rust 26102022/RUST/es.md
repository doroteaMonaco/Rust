### ESERCIZIO 1
- Lo stack è la parte di memoria allocata staticamente secondo una strategia LIFO. La memoria dello stack viene liberata automaticamente.
- Lo heap è la parte di memoria allocata dinamenicamente che contiene i dati. La memoria deve essere liberata dal programmatore.

- Box<[T]> -> nello stack ho un puntatore al primo elemento della slice e un la lunghezza della slice. Nello heap ho gli elementi della slice.
- RefCell<T>: nello stack ho il borrow e il tipo nello stack.
- &[T]: nello stack ho il puntatore al primo elemento della slice e la lunghezza.

### ESERCIZIO 2

In termini di robustezza l'approccio mediante processi risulta migliore in quanto impelementato con isolamento, ma per la scalabilità risulta preferibile il multi thread in quanto uno stesso processo può essere scalato in sistemi large scale su più thread. Un processo con più thread plausibilmente permetterà di parallelizzare i compiti (se si dispone di cpu multicore e la sincronizzazione è correttamente gestita es. non ci sono thread globabi che serializzino i compiti) e dunque svolgerli in minor tempo rispetto ad un approccio mono-thread.

### ESERCIZIO 3
La programmazione multi-thread non è deterministica, in istanti diversi si possono avere output diversi.

Un esempio che dimostra l'imprevedibilità di un approccio multi-thread è quello dell'**interferenza**, possiamo immaginare una istruzione apparentemente innocua come un increment: `a++`. Nonostante sembri una istruzione sola (atomica) di fatto questa si traduce in due istruzioni: `temp=a; a=temp+1;`.