# The Case for the Typestate Pattern - Introducing Algebraic Data Types

The typestate pattern is an approach to designing data types and APIs that is quite popular in the Haskell and Rust ecosystem.
I haven't heard of it in the Java/Kotlin world, though.
So in this blog series, I'll show different representations of the same example program and talk about each versions pros and cons.

I'll use Rust at the beginning and translate the program into Kotlin in a later article to show that this design pattern is also usable in Kotlin.
We'll also take the opportunity to compare Rust with Kotlin.

Let's first introduce the example:

## An example data type with many different states

Let's say our app manages the orders of a bicycle repair shop.
The main data structure of this app is the `RepairOrder`.

A `RepairOrder` can be in the following distinct states:

![New, Invalid, Validated, Work In Progress, Waiting for Payment, Paid/Archived](states.svg)

The validation step from New to Invalid or Validated is the only state transition that can end up in two possible states.
All other state transitions are completely deterministic.

A very straightforward implementation could just distinguish between states using booleans and store all data that is ever needed in the same data type.

## Implementation 1: Lots of simple types

This is the data type:

```rust
struct RepairOrder {
    order_number: u64,
    damage_description: Option<String>,
    vehicle: String,
    customer: Customer,
    valid: Option<bool>,
    validation_errors: Vec<String>,
    assigned_technician: Option<Employee>,
    in_progress: bool,
    steps_left: Vec<String>,
    paid: bool,
    invoice: Option<String>,
}
```

and this is the function that implements the main workflow:

```rust
fn process(mut order: RepairOrder) {
    // 1. Validate
    if validate(&order) {
        order.valid = Some(true);
    } else {
        order.valid = Some(false);
        order.validation_errors = validation_errors(&order);
        store_invalid_order(order);
        return;
    }

    assert_eq!(order.valid, Some(true));

    // 2a. Find a technician to assign
    let free_technician = find_idle_technician();
    order.assigned_technician = Some(free_technician);

    // 2b. Start progress
    order.in_progress = true;

    order.steps_left = calculate_steps();

    while !order.steps_left.is_empty() {
        work_on_next_step(&mut order);
    }
    // Progress is finished

    assert!(order.steps_left.is_empty());

    // 3. Send invoice
    let invoice = send_invoice(&order);
    order.invoice = Some(invoice);

    // Wait for payment
    await_payment();

    // 4. Order is now paid and archived
    order.paid = true;

    assert!(order.paid);
}
```

All actual work is done in the other functions, but all state transitions are visible here.

### Cons

This approach has many weaknesses (it is the 'before' in this 'before vs after' comparison, so this was intentional):

* it's not very obvious from looking at the data structure how many possible states there are
    * One per boolean? 2^number_of_booleans?
* I have to look at the logic that uses the data to see which states are possible and which data is used when
* it is technically possible for the data to be in an invalid state
    * like `!valid && inProgress`, is that legal?
    * is it even correct that `inProgress` stays `true` when the work is done and the billing process starts?
* some fields only apply to some states (the `invoice` field only becomes relevant when the work is finished, but is always available)
* you might also notice that this won't be pleasant to test, but in my opinion this is mostly caused by the method being very large, which is not what I want to compare here.

So when working with this grab-bag of variables there's probably a higher chance of overlooking bugs.

### Pros

* If all variable combinations are actually used, this is the easiest implementation.
* If I had to start writing code only knowing what data exists, but not really what states will realistically exist, I might start with this kind of implementation and hope to refactor it in the future.

## 2: State represented with sum types/enums/sealed classes

Representing the possible states using C-style enums would help make it visible how many states there are and make invalid boolean combinations impossible:

```rust
pub struct RepairOrder {
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub vehicle: String,
    pub customer: Customer,
    pub state: OrderState
    pub validation_errors: Vec<String>,
    pub assigned_technician: Option<Employee>,
    pub steps_left: Vec<String>,
    pub invoice: Option<String>,
}
pub enum OrderState {
    New, Valid, Invalid, InProgress, WorkDone, WaitingForPayment, Paid
}
```

But we would like to also solve the issue that several fields are only relevant in some states.
To do this, we can move those fields from `RepairOrder` into the relevant states using _algebraic data types_ (using enums in Rust or sealed classes in Kotlin).

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

Ah, looks cleaner already.

### Functions

The function implementations change a little.
We're gonna avoid a big procedural block this time and move each step into a method.

The main function now mostly consists of state transitions:

```rust
pub fn process(mut order: RepairOrder) {
    assert_eq!(order.state, State::New);

    order.validate();

    if order.is_invalid() {
        return;
    }

    let technician = find_idle_technician();
    let steps_left = calculate_steps();
    order.start_progress(technician, steps_left);

    order.work();
    order.send_invoice();
    order.await_payment();
}
```

We're also only going to look at two functions, the entire example code is linked (here)[https://github.com/TimoFreiberg/typestate-post/blob/master/typestate-rust/src/repair_order/state_enum.rs].

#### Validate Function

```rust
impl RepairOrder {
    //...
    fn validate(&mut self) {
        self.state = if self.is_valid() {
            State::Valid
        } else {
            let validation_errors = get_validation_errors();
            State::Invalid { validation_errors }
        };
    }
    //...
}
```

This one only gets more descriptive compared to the initial version, which is helped by the fact that it doesn't validate the initial state.

#### Work Function

```rust
    //...
    fn work(&mut self) {
        while self.has_steps_left() {
            self.work_on_next_step()
        }
    }
    fn has_steps_left(&self) -> bool {
        let steps_left = match &self.state {
            State::InProgress { steps_left, .. } => steps_left,
            other => panic!("Expected InProgress, but was {:?}", other),
        };
        !steps_left.is_empty()
    }
    //...
```

Here, the current state actually has to be `InProgress` for the function to work, and handling that is a bit more complex.

#### Pros

* It's immediately visible how many states there are.
* Only one state can be active at once
* States don't have access to data of other states.
* We can now make fields like `assigned_technician` mandatory, which wasn't possible in the previous approach!

#### Con

* Accessing the expected state is a bit cumbersome, requiring a `match` every time which needs to either return an error or panic if the wrong state was used.

If this con seems like a drawback of adding type explicitness, let me make the case for going even further:

## The case for going even further: The typestate pattern

This might come as a surprise, but the example application was designed to fit the typestate pattern particularly well.
Therefore, the version using the typestate pattern will have the same benefits as the states-as-enums version (even partially improving on them), without the drawback.

Before introducing the final version though, let's first talk about why the version using enums is suboptimal for this specific algorithm.

The state transitions of the order are deterministic and visible to the human reader, who can see what the expected state at each line of the program will be.
Any effort required to verify that the `state` field actually contains the expected state can feel like annoying overhead that the type checker is forcing upon the human.

But the typechecker only requires these checks because the state field could be changed at any point - a power we don't need!
Maybe we could give up these powers, restrict the functions to only accept correct states and make our lives easier...

One way to do this without using any modern language features is to introduce completely separate types for each state.
In that case, `validate` would receive a `NewRepairOrder` and return a `Result<ValidRepairOrder, InvalidRepairOrder>`.
Similarly, `work` would be a mutating method on `InProgressRepairOrder`.  
This is definitely a valid approach in some places, but mostly way too verbose and clunky.

Read the next article to see the typestate pattern improve everything, and for a discussion on when to choose which approach.

---

The full example code is available [here](https://github.com/TimoFreiberg/typestate-post/blob/master/typestate-rust/src/repair_order/state_enum.rs).
