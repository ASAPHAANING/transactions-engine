# Transactions Engine
This project stores the source code for an application that accepts a series of transactions and produces a list of balances
for the distinct client accounts represented amongst the transactions. Both input and output formats are CSV.

Run with cargo by forwarding the CLI arguments using `--` like so:

```
➜ cargo run -- fixtures/tx1.csv
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/transactions-engine fixtures/tx1.csv`
client,available,held,total,locked
2,2,0,2,false
1,1.5,0,1.5,false
```

The application writes to stdout and as such can pipe the output to a destination of choice.

## Considerations
**Streaming**
I spent some time thinking about how to achieve lazy-evaluated streaming of the event-sourced transactions.
Iterators (preferably asynchronous ones) sprung to mind but do not exactly fulfil the ideal design.

Further work in this area could include constructing a custom kind that allows for high-watermarks in case chunk-streaming is required,
allowing for backpressure to define the memory limitations of the engine.

In order for the investment to be worth setting it up, a means to quickly look up transactions (as required by the business logic)
should also be provided such that every transaction pertained to a client account is stored in memory.

I imagine in an indexed database with a hot cache.

*Structure*
Defining the models of the application, a core component of that was the *transaction*. I originally sought to define it as follows:

```
enum Transaction {
    Withdrawal { id: u32, .. },
    ..
}
```

but could not get the deserializer to work with the input format. It would have been possible to write a custom deserializer given more time.

## Testing
In the `Output` section of the document, invariant definitions are provided along with the meaning of the columns.
These can be used to setup a rigorous test suite that ensures changes never violate these invariants. (`available = total - held, ...`)

The unit tests of a third party library is unconvential but was kept in to show the process of verifying my understanding of the documentation.

Fixtures provide a convenient way of testing some fresh-out-the-oven data sets directly against the binary.
