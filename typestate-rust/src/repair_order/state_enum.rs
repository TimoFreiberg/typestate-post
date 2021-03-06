use super::{calculate_steps, find_idle_technician, Customer, Employee};

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

#[derive(Debug)]
pub struct RepairOrder {
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub vehicle: String,
    pub customer: Customer,
    pub state: State,
}

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    New,
    Valid,
    Invalid {
        validation_errors: Vec<String>,
    },
    InProgress {
        assigned_technician: Employee,
        steps_left: Vec<String>,
    },
    WorkDone,
    WaitingForPayment {
        invoice: String,
    },
    Paid {
        invoice: String,
    },
}

impl RepairOrder {
    fn validate(&mut self) {
        let is_valid = is_valid();
        self.state = if is_valid {
            State::Valid
        } else {
            let validation_errors = get_validation_errors();
            State::Invalid { validation_errors }
        };
    }
    fn start_progress(&mut self, technician: Employee, steps_left: Vec<String>) {
        assert_eq!(self.state, State::Valid);
        self.state = State::InProgress {
            steps_left,
            assigned_technician: technician,
        };
    }
    fn work(&mut self) {
        while self.has_steps_left() {
            self.work_on_next_step()
        }
        self.state = State::WorkDone;
    }
    fn has_steps_left(&self) -> bool {
        let steps_left = match &self.state {
            State::InProgress { steps_left, .. } => steps_left,
            other => panic!("Expected InProgress, but was {:?}", other),
        };
        !steps_left.is_empty()
    }
    fn send_invoice(&mut self) {
        let invoice = get_invoice();
        self.state = State::WaitingForPayment { invoice };
    }
    fn await_payment(&mut self) {
        let invoice = match &self.state {
            State::WaitingForPayment { invoice, .. } => invoice.clone(),
            other => panic!("Expected WaitingForPayment, got {:?}", other),
        };
        await_payment();
        self.state = State::Paid { invoice };
    }
    fn work_on_next_step(&mut self) {
        todo!()
    }
    fn is_invalid(&self) -> bool {
        match self.state {
            State::Invalid { .. } => true,
            _ => false,
        }
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
