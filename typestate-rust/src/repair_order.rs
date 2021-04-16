use serde::{Deserialize, Serialize};

pub mod complex_state;
pub mod single_variables;
pub mod state_enum;
pub mod typestate;
pub mod typestate_serialization_every_step;
pub mod typestate_serialization_start_end;

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    has_outstanding_debt: bool,
    is_banned: bool,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Employee;

fn find_idle_technician() -> Employee {
    todo!()
}

fn calculate_steps() -> Vec<String> {
    todo!()
}
