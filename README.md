Streaming took a long time..

Wanted to do

```
enum Transaction {
    Withdrawal { id: u32, .. },
    ..
}
```
but could not get the deserializer to work with the input format.
It would have been possible to write a custom deserializer given more time.

## Testing
In the `Output` section of the document, invariant definitions are provided along with the meaning of the columns.
These can be used to setup a rigorous test suite that ensures changes never violate these invariants. (`available = total - held, ...`)

The unit tests of a third party library is unconvential but was kept in to show the process of verifying my understanding of the documentation.

Fixtures provide a convenient way of testing some fresh-out-the-oven data sets directly against the binary.
