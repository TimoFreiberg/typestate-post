# The Case for the Typestate Pattern - Actually It Depends

So far, I introduced several approaches to designing types to represent states in Rust.
I presented them in a clear order from worst to best, but this ranking is .

There are several reasons why the typestate pattern suited the algorithm so well.

## Simple state transitions

One of them is that the state transitions looked like this:

image::states.svg["New, Invalid, Validated, Work In Progress, Waiting for Payment, Paid/Archived"]

Note that most states are followed by exactly a single other state - the Invalid state even represents an early exit which was easy to model with a `return` (or a `?`).

Function signatures for state transitions looked equivalent to this:

```rust
fn(RepairOrder<Valid>) -> RepairOrder<InProgress>
```

or at its most complex:

```rust
fn(RepairOrder<New>) -> Result<RepairOrder<Valid>, RepairOrder<Invalid>>
```

How would this look if the states looked more like this?
 
image::complex-states.svg["A complicated web of states with silly names"]

The first function would look more like the following:

```rust
fn(RepairOrder<New>) -> OneOf4<RepairOrder<Invalid>, RepairOrder<LowPriority>, RepairOrder<HighPriority>, RepairOrder<AprilFools>>
```

Now the approach looks a lot less fun to implement.
And we haven't even looked at a function that chains each state transition together:

```rust

```
