# The Case for the Typestate Pattern - The Typestate Pattern itself

In the [previous article](https://www.novatec-gmbh.de/en/blog/the-case-for-the-typestate-pattern-introducing-algebraic-data-types/), I showed two equivalent implementations of the same program to compare different approaches of encoding state in types.

Now, I'll show the typestate pattern.

## Quick refresher

The data structure of the previous version looked like this:

```rust
pub struct RepairOrder {
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub vehicle: String,
    pub customer: Customer,
    pub state: OrderState
}
pub enum OrderState {
    New,
    Valid,
    Invalid { validation_errors: Vec<String> },
    InProgress {
        assigned_technician: Employee,
        steps_left: Vec<String>
    },
    WorkDone,
    WaitingForPayment { invoice: String },
    Paid { invoice: String }
}
```

And the main issue with it was that each function had to pattern match over all possible variants of `OrderState` when doing anything with `state`, although all functions expected to always be called with a single state.

To improve that, we just need to give each state its own type.
But instead of having separate `NewRepairOrder`, `InvalidRepairOrder`, ... types, we can use types like `RepairOrder<New>` and `RepairOrder<InProgress>`.
The data structures look quite similar to the previous version:

```rust
#[derive(Debug)]
pub struct RepairOrder<State> {
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub vehicle: String,
    pub customer: Customer,
    pub state: State,
}
struct New;
struct Valid;
struct Invalid { validation_errors: Vec<String> }
struct InProgress {
    assigned_technician: Employee,
    steps_left: Vec<String>,
}
struct WorkDone;
struct WaitingForPayment { invoice: String }
struct Paid { invoice: String }
```

Very similar, but now the states are not part of the same type but their own separate types.
Let's look at how some of the state-transitioning functions look:

```rust
impl RepairOrder<New> {
    fn validate(self) -> Result<RepairOrder<Valid>, RepairOrder<Invalid>> {
        let is_valid = is_valid();
        if is_valid {
            Ok(self.with_state(Valid))
        } else {
            let validation_errors = get_validation_errors();
            Err(self.with_state(Invalid { validation_errors }))
        }
    }
}
impl RepairOrder<InProgress> {
    fn work(mut self) -> RepairOrder<WorkDone> {
        while self.has_steps_left() {
            self.work_on_next_step()
        }
        self.with_state(WorkDone)
    }
    fn has_steps_left(&self) -> bool {
        self.state.steps_left.is_empty()
    }
}
```

So the state is now visible in the types, allowing us to directly access its data without any further checks.
The type signatures also make all state transitions very visible, which could help understand more complicated state machines.

You might have noticed that I dont set the state via `self.state = new_state`, instead using a helper function, which looks like this:

```rust
impl<State> RepairOrder<State> {
    fn with_state<NewState>(self, new_state: NewState) -> RepairOrder<NewState> {
        RepairOrder {
            order_number: self.order_number,
            damage_description: self.damage_description,
            vehicle: self.vehicle,
            customer: self.customer,
            state: new_state,
        }
    }
}
```

This is unfortunately necessary to change the state type parameter.[^state-type-change]

The main function still only consists of state transitions:

```rust
pub fn process(order: RepairOrder<New>) {
    let valid = match order.validate() {
        Ok(valid) => valid,
        Err(_) => {
            return;
        }
    };

    let technician = find_idle_technician();
    let steps_left = calculate_steps();
    let in_progress = valid.start_progress(technician, steps_left);

    let done = in_progress.work();
    let waiting = done.send_invoice();
    let done = waiting.await_payment();
}
```

In comparison to the previous version, intermediate variables have to be used here as each new state is the return value of the previous call.
I criticised that aspect of the previous version, but surely having to give intermediate steps a name is fine?

Well, let's just show the final, optimal form of the `process` function and admire its beauty.

```rust
pub fn process_the_ultimate(
    order: RepairOrder<New>,
) -> Result<RepairOrder<Paid>, RepairOrder<Invalid>> {
    Ok(order
        .validate()?
        .start_progress(find_idle_technician(), calculate_steps())
        .work()
        .send_invoice()
        .await_payment())
}
```

With small changes to its API to make it more realistic and allow use of the `?` operator.  
You know, this was intended as a joke, but I actually think this is the nicest version so far.

### Pros

* All of the previous version
* More specific type signatures
* Allows method chaining
    * Ok this is a bit unfair, the previous version could have also offered a fluent API
        It's very natural in this version, though
* No boilerplate state unpacking

### Con

* One boilerplate state transitioning function is required

## So is this just better?

Nope, it's all situational.
The specific algorithm I made up suits the typestate pattern well.

I'm going to give some examples where each approach works best in the next article.

## Only in Rust? (Or Haskell?)

While I have mostly heard of the typestate pattern in the Rust and Haskell community, the examples in this post are easily translatable into Kotlin, which I will showcase later in this series.

## Further reading

A more in-depth look at the way Rust's type system helps representing state: http://cliffle.com/blog/rust-typestate/

The motivation behind an unusually type-safe path handling library, written in Haskell (which blew my mind at the time): https://chrisdone.com/posts/path-package/

---

[^state-type-change]: The same verbose workaround is required in Kotlin. Haskell does not have the same issue, mostly because there's 
