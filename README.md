upget
===

Super simple trait that patterns the value "updae" and "get".

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

The `Upget` trait only has an `upget` method, which just updates `self` value
with a closure and then retrieves it. That's all there is to it, but it allows
you to refactor common code patterns.

## Target Code

For example, the following code can be refactored...

```rust
fn clone_with_sort(vec: &Vec<String>) -> Vec<String> {
    let mut result = vec.clone();
    result.sort();
    result
}
```

as follows.

```rust
fn clone_with_sort(vec: &Vec<String>) -> Vec<String> {
    vec.clone().upget(|x| x.sort())
}
```

Such refactoring has the following benefits.
* Simplification of code.
* Elimination of intermediate variables.
* Elimination of the `mut` specification.

## Versions

See [CHANGELOG](CHANGELOG.md).

