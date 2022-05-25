# TOY machine simulator

A Rust port of the TOY machine simulator, as described and explained by [Sedgewick and Wayne](https://www.comscigate.com/cs/IntroSedgewick/50machine/52toy/index.html). Their original implementation can be found at [Toy.java](https://www.comscigate.com/cs/IntroSedgewick/50machine/55simulator/index.html).

All of their provided [examples](https://www.comscigate.com/cs/IntroSedgewick/50machine/54programming/index.html) work as intended and will be added here later, too.

The simplest example is [`add.toy`](https://www.comscigate.com/cs/IntroSedgewick/50machine/54programming/add.toy):
```
// add.toy
// Input: Stored in memory location 00 and 01
// Output: Sum of two integers saved in memory location 02
// Remarks: 

00: 0008 8
01: 0005 5

10: 8A00 r[A] <- mem[00]
11: 8B01 r[B] <- mem[01]
12: 1CAB r[C] <- r[A] + r[B]
13: 9C02 mem[02] <- r[C]
14: 0000 halt
```

To run the simulator:
```
$ cargo run add.toy
```
