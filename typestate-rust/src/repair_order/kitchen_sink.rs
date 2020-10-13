use std::thread;

use time::NumericalStdDurationShort;

use super::{Customer, Employee};

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

    while order.assigned_technician.is_none() {
        match find_idle_technician() {
            Some(it) => order.assigned_technician = Some(it),
            None => {
                thread::sleep(30.minutes());
            }
        }
    }

    assert!(order.assigned_technician.is_some());

    order.in_progress = true;

    // TODO add steps, add a way to block, add inventory handling (blocked for items/part of steps), add payment handling
}

#[derive(Debug)]
pub struct RepairOrder {
    pub order_number: u64,
    pub damage_description: Option<String>,
    pub is_inspection: bool,
    pub vehicle: String,
    pub customer: Customer,
    pub valid: Option<bool>,
    pub validation_errors: Vec<String>,
    pub assigned_technician: Option<Employee>,
    pub in_progress: bool,
    pub steps_left: Vec<String>,
    pub blocked: bool,
    pub waiting_for_items: Vec<String>,
    pub paid: bool,
    pub invoice: Option<String>,
}

impl RepairOrder {
    pub fn new(
        customer: Customer,
        damage_description: Option<String>,
        is_inspection: bool,
        vehicle: String,
    ) -> Self {
        let order_number = generate_order_number();
        RepairOrder {
            order_number,
            damage_description,
            is_inspection,
            vehicle,
            customer,
            valid: None,
            validation_errors: Vec::new(),
            assigned_technician: None,
            in_progress: false,
            steps_left: Vec::new(),
            blocked: false,
            waiting_for_items: Vec::new(),
            paid: false,
            invoice: None,
        }
    }
}

fn generate_order_number() -> u64 {
    // chosen by fair dice roll.
    4
}

fn find_idle_technician() -> Option<Employee> {
    Some(Employee)
}

fn store_invalid_order(order: RepairOrder) {
    println!("Please store me: {:?}", order);
}

fn validation_errors(order: &RepairOrder) -> Vec<String> {
    let mut errors = Vec::new();
    if order.customer.has_outstanding_debt {
        errors.push("Customer has outstanding debt".into());
    }
    if order.customer.is_banned {
        errors.push("Customer is banned from the shop".into());
    }
    errors
}

fn validate(order: &RepairOrder) -> bool {
    !order.customer.has_outstanding_debt && !order.customer.is_banned
}
