# The Case for the Typestate Pattern - The Typestate Pattern itself

In the [previous article](https://www.novatec-gmbh.de/en/blog/the-case-for-the-typestate-pattern-introducing-algebraic-data-types/), I showed two equivalent implementations of the same program to compare different approaches of encoding state in types.

Now, I'll show the typestate pattern.

### Quick refresher

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

And there was one issue I had with that approach:
Every function that expected a particular state had to perform the same annoying pattern matching:

```rust
match &self.state {
    State::Valid => { /* do something */ }
    other => panic!(),
};
```

To improve that, we can give each state its own type.
But instead of having separate `NewRepairOrder`, `InvalidRepairOrder` (etc) types, we can use the state both as a field and as a type parameter.
This changes the corresponding types into `RepairOrder<New>` and `RepairOrder<Invalid>`.
The data structures look quite similar to the previous version:

```rust
#[derive(Debug)]
pub struct RepairOrder<State> { // <- Added type parameter
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub vehicle: String,
    pub customer: Customer,
    pub state: State, // <- this value is now completely polymorphic
}
struct New; // <- One of our states
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
The type signatures also make all state transitions very visible, which could make complicated state machines more readable.

You might have noticed that I don't set the state via `self.state = new_state`, instead using a helper function, which looks like this:

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

This is unfortunately necessary to change the state type parameter[^state-type-update].

The main function still only consists of state transitions, 
but now the return values of all the functions need to be used for each next step.

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

I criticised the verbosity of the previous version, but surely having to give intermediate steps a name is fine?
Well, we actually don't have to, because returning the next state gave us a fluent API.
Using method chaining shortens the function drastically:


```rust
pub fn process(
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

I had to change the function signature slightly to use the [`?` operator](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html), but I think this only makes it more realistic.

The method chaining might not be everyones cup of tea, but I actually think this is the prettiest version so far.

#### Pros

* All pros of the previous version
* More specific type signatures
* Allows method chaining
    * Ok this is a bit unfair, the previous version could have also offered a fluent API  
    It's very natural in this version, though
* No boilerplate state unpacking with `match state {...}`

#### Con

* One boilerplate state transitioning function is required

### Sounds awesome, should I use this pattern everywhere now?

Well, it depends.
I would argue that for this specific algorithm I made up, the typestate pattern would be a good idea.

In other situations, it might be a very bad idea though.
I'm going to give some examples where other approaches work much better in the next article.

### Only in Rust? (Or Haskell?)

I have mostly heard of the typestate pattern in the Rust and Haskell community.
But the examples in this post are easily translatable into Kotlin, which I'm going to show in a future article.

### Further reading

I can recommend these articles:

[http://cliffle.com/blog/rust-typestate/](http://cliffle.com/blog/rust-typestate/) - a more in-depth look at the way Rust's type system helps representing state.

[https://chrisdone.com/posts/path-package/](https://chrisdone.com/posts/path-package/) - the motivation behind an unusually type-safe path handling library, written in Haskell (which blew my mind at the time).

---

[^state-type-update]: The same verbose workaround is required in Kotlin. Haskell does not have the same issue, but then there's no in-place mutation in Haskell. There's a Rust [RFC](https://github.com/rust-lang/rfcs/pull/2528) in progress to improve this exact interaction.
