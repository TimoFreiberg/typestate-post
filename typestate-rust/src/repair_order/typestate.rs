use serde::{Deserialize, Serialize};

use super::{calculate_steps, find_idle_technician, Customer, Employee};

pub fn process(order: RepairOrder<New>) {
    // assert_eq!(order.state, State::New);

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
    let _done = waiting.await_payment();
}

pub fn process_fluent(order: RepairOrder<New>) -> Result<RepairOrder<Paid>, RepairOrder<Invalid>> {
    Ok(order
        .validate()?
        .start_progress(find_idle_technician(), calculate_steps())
        .work()
        .send_invoice()
        .await_payment())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepairOrder<State> {
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub vehicle: String,
    pub customer: Customer,
    #[serde(flatten)]
    pub state: State,
}

#[derive(Deserialize)]
pub struct New;
pub struct Valid;
#[derive(Debug, Serialize)]
pub struct Invalid {
    pub validation_errors: Vec<String>,
}
pub struct InProgress {
    pub assigned_technician: Employee,
    pub steps_left: Vec<String>,
}
pub struct WorkDone;
pub struct WaitingForPayment {
    pub invoice: String,
}
#[derive(Serialize)]
pub struct Paid {
    pub invoice: String,
}

impl<State> RepairOrder<State> {
    pub fn with_state<NewState>(self, new_state: NewState) -> RepairOrder<NewState> {
        RepairOrder {
            order_number: self.order_number,
            damage_description: self.damage_description,
            vehicle: self.vehicle,
            customer: self.customer,
            state: new_state,
        }
    }
}

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
impl RepairOrder<Valid> {
    fn start_progress(
        self,
        technician: Employee,
        steps_left: Vec<String>,
    ) -> RepairOrder<InProgress> {
        self.with_state(InProgress {
            steps_left,
            assigned_technician: technician,
        })
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
    fn work_on_next_step(&mut self) {
        todo!()
    }
}

impl RepairOrder<WorkDone> {
    fn send_invoice(self) -> RepairOrder<WaitingForPayment> {
        let invoice = get_invoice();
        self.with_state(WaitingForPayment { invoice })
    }
}
impl RepairOrder<WaitingForPayment> {
    fn await_payment(self) -> RepairOrder<Paid> {
        let invoice = self.state.invoice.clone();
        await_payment();
        self.with_state(Paid { invoice })
    }
}

fn await_payment() {
    todo!()
}

fn get_invoice() -> String {
    todo!()
}

fn get_validation_errors() -> Vec<String> {
    todo!()
}

fn is_valid() -> bool {
    todo!()
}
