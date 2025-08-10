### ESERCIZIO 1
Le dipendenze cicliche sono due puntatori che si puntano a vicenda. Il problema sorge solo quando i puntatori sono entrambi Rc poichè i contatori si incrementano entrambi e non si azzerano mai.
### ESERCIZIO 2
std::process permette di creare, gestire processi esterni.
La creazione di un processo avviene attraverso il tipo Command -> new().arg().output().expect();
L'esecuzione asincrona avviene attraverso .spawn() che lo fa iniziare senza aspettare che termini.
il metodo .wait() permette di aspettare la sua terminazione.
Per gestire direttamente il processo figlio si chiama il tipo Child, che al suo interno ha il metodo wait_pid();

### ESERCIZIO 3
In un contesto di programmazione concorrente si può evitare la condizione di deadlock semplicemente rendendo accessibile quella risorsa a tutti thread gestendo l'accesso concorrente, ovvero usando il costrutto Arc<Mutex<>>. Eventualemte si può gestire l'attesa con una Condtion Variable. Arc implementa sia il tratto Sync sia il tratto Send, quindi permette l'accesso concorrente immutabile (mutabile tramite Mutex) e Send lo rende condivisibile.
