## Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## References

Aampersands are references, and they allow you to refer to some value without taking ownership of it.

Having references as function parameters is called **borrowing**

Mutable references have one big restriction: you can only have one mutable reference to a particular piece of data in a particular scope.

Summary:

* At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
* References must always be valid.