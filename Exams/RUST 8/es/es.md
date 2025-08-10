
- data:
    - STACK-> puntatore + capacity + len = 8 + 8 + 8 = 24B
- threshold: 
    - STACK -> tag + padding + T = 1 + 4 + 3 = 8B

- Bucket:
    - STACK -> 32B


308a6e01 00600000 03000000 00000000 03000000 00000000 01000000 0a000000 -> sono blocchi da 32 bit = 32B -> CORRECT

Lo tsack è LIFO, quindi l'ultimo ad essere inserito è nel TOS.

000000a0 (i32) e 00000010 (tag) sono il threashold

00000000 00000030 -> capacity di data
00000000 00000030 -> len di data
00000600 10e6a803 -> puntatore di data
