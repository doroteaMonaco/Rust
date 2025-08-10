## ESERCIZIO 1
Gli smart pointer sono dei puntatori intelligenti con funzionalità aggiuntive come ad esempio una gestione di contatori ai riferimenti per capire quando può essere rilasciato.
La gestione dei conteggi può essere implementata da Rc e weak, entrambi infatti sono caratterizzati da due counter, uno per i riferimenti forti e uno per i riferiemnti deboli. Quando il counter dei riferimenti forti arriva a 0, Rc può essere rilasciato.
Un altro esempio è Box<T>.

## ESERCIZIO 2
Per std::channel() si intende un canale asincrono con molteplici trasmettitori e un ricevitore. Viene normalmente ustao nella programmazione concorrente per gestire la comunicazione tra thread e prevede due metodi principali: send() per l'invio del messaggio e recv() per la ricezione bloccante, il ricevitore rimane in attesa fino a quando non lo riceve.

Per std::sync_channel() si intende un canale sincrono con molteplici Sync Senders e molteplici ricevitori. Il canali infatti può essere definito come bounded o unbounded a seconda che ci sia una soglia o meno pe ril numero massimo di messaggi transitabili nel canale. La send in questo caso può essere bloccante: se il sender vuole inviare il messaggio ma il buffer è pieno, aspetta che si svuoti.

## ESERCIZIO 3

```rust
1.  struct Point {
2.      x: i16,
3.      y: i16,
4.  }
5.  
6.  enum PathCommand {
7.      Move(Point),
8.      Line(Point),
9.      Close,
10. }
11. let mut v = Vec::<PathCommand>::new();
12. v.push(PathCommand::Move(Point{x:1,y:1}));
13. v.push(PathCommand::Line(Point{x:10, y:20}));
14. v.push(PathCommand::Close);
15. let slice = &v[..];
```
#### Point 
Stack -> 2B * 2 = 4B;

#### PathCommand
Stack -> 1B + 4B = 5B + 3B = 8B;

11) Stack -> 8B + 8B + 8B = 24B
12) Heap -> 8B;
13) Heap -> 8B
14) Heap -> 8B
15) Stack -> 8B + 8B = 16B

- STACK -> 40B
- HEAP -> 24B

