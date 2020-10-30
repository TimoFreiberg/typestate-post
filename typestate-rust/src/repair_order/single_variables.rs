use super::{calculate_steps, find_idle_technician, Customer, Employee};

pub fn process(mut order: RepairOrder) {
    if order.is_valid() {
        order.valid = Some(true);
    } else {
        order.valid = Some(false);
        order.validation_errors = order.validation_errors();
        return;
    }

    assert_eq!(order.valid, Some(true));

    let free_technician = find_idle_technician();
    order.assigned_technician = Some(free_technician);

    order.in_progress = true;

    order.steps_left = calculate_steps();
    while !order.steps_left.is_empty() {
        order.work_on_next_step();
    }
    assert!(order.steps_left.is_empty());

    let invoice = send_invoice(&order);
    order.invoice = Some(invoice);

    await_payment();
    order.paid = true;

    assert!(order.paid);
}

#[derive(Debug)]
pub struct RepairOrder {
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub vehicle: String,
    pub customer: Customer,
    pub valid: Option<bool>,
    pub validation_errors: Vec<String>,
    pub assigned_technician: Option<Employee>,
    pub in_progress: bool,
    pub steps_left: Vec<String>,
    pub paid: bool,
    pub invoice: Option<String>,
}

impl RepairOrder {
    pub fn new(
        order_number: u64,
        damage_description: Option<String>,
        vehicle: String,
        customer: Customer,
    ) -> Self {
        Self {
            order_number,
            damage_description,
            vehicle,
            customer,
            valid: None,
            validation_errors: Vec::new(),
            assigned_technician: None,
            in_progress: false,
            steps_left: Vec::new(),
            paid: false,
            invoice: None,
        }
    }
    fn is_valid(&self) -> bool {
        todo!()
    }
    fn validation_errors(&self) -> Vec<String> {
        todo!()
    }
    fn work_on_next_step(&mut self) {
        todo!()
    }
}

fn send_invoice(_order: &RepairOrder) -> String {
    todo!()
}

fn await_payment() -> bool {
    todo!()
}
