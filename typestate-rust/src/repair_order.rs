pub mod single_variables;
pub mod state_enum;
pub mod typestate;

#[derive(Debug)]
pub struct Customer {
    has_outstanding_debt: bool,
    is_banned: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Employee;

fn find_idle_technician() -> Employee {
    todo!()
}

fn calculate_steps() -> Vec<String> {
    todo!()
}
