Streaming took a long time.

Wanted to do

```
enum Transaction {
    Withdrawal { id: u32, .. },
    ..
}
```
but could not get the deserializer to work with the input format.

## Testing
In the `Output` section of the document, invariant definitions are provided along with the meaning of the columns.
These can be used to setup a rigorous test suite that ensures changes never violate these invariants. (`available = total - held, ...`)
