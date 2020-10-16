use super::{calculate_steps, find_idle_technician, Customer, Employee};

pub fn process(mut order: RepairOrder) {
    if validate(&order) {
        order.valid = Some(true);
    } else {
        order.valid = Some(false);
        order.validation_errors = validation_errors(&order);
        store_invalid_order(order);
        return;
    }

    assert_eq!(order.valid, Some(true));

    let free_technician = find_idle_technician();
    order.assigned_technician = Some(free_technician);

    order.in_progress = true;

    order.steps_left = calculate_steps();

    while !order.steps_left.is_empty() {
        work_on_next_step(&mut order);
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
    pub fn new(customer: Customer, damage_description: Option<String>, vehicle: String) -> Self {
        let order_number = generate_order_number();
        RepairOrder {
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
}

fn generate_order_number() -> u64 {
    todo!()
}

fn store_invalid_order(_order: RepairOrder) {
    todo!()
}

fn validation_errors(_order: &RepairOrder) -> Vec<String> {
    todo!()
}

fn validate(_order: &RepairOrder) -> bool {
    todo!()
}

fn work_on_next_step(_order: &mut RepairOrder) {
    todo!()
}

fn await_payment() -> bool {
    todo!()
}

fn send_invoice(_order: &RepairOrder) -> String {
    todo!()
}
