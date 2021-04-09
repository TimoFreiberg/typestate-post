use typestate::{New, RepairOrder};

use super::typestate;

pub fn process(request: String) -> eyre::Result<String> {
    let new_order: RepairOrder<New> = serde_json::from_str(&request)?;
    Ok(match typestate::process_fluent(new_order) {
        Ok(paid) => serde_json::to_string(&paid),
        Err(invalid) => serde_json::to_string(&invalid),
    }?)
}
