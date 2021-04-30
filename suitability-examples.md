# The Case for the Typestate Pattern - Actually It Depends

TODO ~tilde surrounded parts~ should be rephrased

So far, I introduced several approaches on how to represent states in Rust.
I presented them in a clear order from worst to best, but this ~is not an objective~ ordering.
I simply chose an algorithm that suited the typestate pattern well, for a few reasons:

## Simple state transitions

The first reason is that the state transitions looked like this:

image::states.svg["New, Invalid, Validated, Work In Progress, Waiting for Payment, Paid/Archived"]

Note that most states transition into exactly one new state.
Only the `New` state can transition to either `Validated` or `Invalid`, but the `Invalid` state leads to an early exit which is easy to model with a `return` (or a `?`).

Function signatures for state transitions looked equivalent to this:

```rust
fn(RepairOrder<Valid>) -> RepairOrder<InProgress>
```

or at its most complex:

```rust
fn(RepairOrder<New>) -> Result<RepairOrder<Valid>, RepairOrder<Invalid>>
```

This made it quite easy to apply the pattern.

## No IO between the states

The data is always received in the same initial state, `New`.
The intermediate states never leave the process memory.
This means that there is no need to ever validate that, e.g. a `RepairOrder<WaitingForPayment>` is still in its correct state, as the data is not passed anywhere that could change the state[^immutable-language].

## Ruining the typestate pattern with complex state transitions

How would this look if the states looked more like this?
 
image::complex-states.svg["A complicated web of states with silly names"]

The first function would look more like the following:

```rust
fn(RepairOrder<New>) -> OneOf4<RepairOrder<Invalid>, RepairOrder<LowPriority>, RepairOrder<HighPriority>, RepairOrder<AprilFools>>
```

Now the approach looks a lot less fun to implement.
And we haven't even looked at a function that chains each state transition together:

```rust
fn process(order: RepairOrder<New>) -> EndStates {
    match order.validate() {
        OneOf4::A(invalid) => {
            match invalid.recover() {
                Ok(recovered) => {
                    todo!()
                }
                Err(krangled) => {
                    todo!()
                }
            }
        },
        OneOf4::B(low_prio) => {
            match low_prio.enqueue() {
                Ok(waiting_for_worker) => {
                    todo!()
                }
                Err(high_prio) => {
                    todo!()
                }
            }
        }
        OneOf4::C(high_prio) => {
            todo!()
        }
        OneOf4::D(april_fools) => {
            todo!()
        }
    }
}
```

Note that I did both of us a favor by stopping pretty early in this body.
Obviously, a good implementation would extract everything 

It's a lot less pleasant to work with this.

## State stays in-process all the time

In the comments to the previous article, someone asked a very good [question](https://www.reddit.com/r/rust/comments/m7nox4/the_case_for_the_typestate_pattern_the_typestate/grf0sle/?utm_source=reddit&utm_medium=web2x&context=3):
How would we save this in a database?

The typestate version uses the type system to statically distinguish between different states.
This doesn't work if the data leaves the process, though.
It doesn't really matter whether it's stored in a database, sent through the network as JSON, stored in a binary format in a file, etc.
As soon as the data leaves the process, we need to represent the state as a value again.
And when data is read from outside the process, the state needs to be part of the data and the data needs to be validated.

To make this a bit more concrete, let's look at a few possible variants.

#### External state 1: De-/Serialization at the start and the end

In this case, we receive a new `RepairOrder` as a `String` containing JSON and return a paid `RepairOrder` as a `String containing JSON.
All the steps inbetween stay exactly the same.
This isn't too bad, we just wrap the previous `fn(RepairOrder<New>) -> Result<RepairOrder<Paid>, RepairOrder<Invalid>>` function in a `fn(String) -> eyre::Result<String>`, like this:

```rust
pub fn process(request: String) -> eyre::Result<String> {
    let new_order: RepairOrder<New> = serde_json::from_str(&request)?;
    Ok(match typestate::process_fluent(new_order) {
        Ok(paid) => serde_json::to_string(&paid),
        Err(invalid) => serde_json::to_string(&invalid),
    }?)
}
```

In this case, the incoming data is expected to be in the `New` state and the returned data in the `Paid` or `Invalid` state, so we can just use the automatic de-/serialization implementations here, which doesn't include the name of the state anywhere.

#### External state 2: De-/Serialization at any step

In this case, every state transition is persisted to a database transactionally.
This could be realistic if the data is important, transitions can take a while and every state transition triggers side effects.

TODO code example

In this case, we can either keep the typestate pattern, create a separate query for every state and have single-step functions that go DB -> specific step -> DB

or 

go back to all states in an enum, create a query that just loads a batch of work and have single-step functions that go DB -> generic next step -> DB

It totally depends which version is better.


# TODO

any more external state possibilities?
any more aspects other than state complexity and out-of-process interactions?

----

[^immutable-language]: It helps that Rust has a strict type system and controlled mutability so it's not possible to do something like that by mistake.
It would be at least possible in C or JavaScript.
