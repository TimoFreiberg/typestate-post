use serde::{Deserialize, Serialize};

use super::typestate::{
    self, InProgress, Invalid, New, Paid, RepairOrder, Valid, WaitingForPayment, WorkDone,
};

pub fn process(request: String) -> eyre::Result<String> {
    let new_order: RepairOrder<New> = serde_json::from_str(&request)?;
    Ok(match typestate::process_fluent(new_order) {
        Ok(paid) => serde_json::to_string(&paid),
        Err(invalid) => serde_json::to_string(&invalid),
    }?)
}

fn store_new(order: RepairOrder<New>) -> eyre::Result<()> {
    // this json doesn't include "new" anywhere
    // store(&serde_json::to_vec(&order)?)?;
    store_with_explicit_state(order)
}

fn store_with_explicit_state(order: RepairOrder<impl Into<States>>) -> eyre::Result<()> {
    store(&serde_json::to_vec(&order.update_state(Into::into))?)
}

fn store(bytes: &[u8]) -> eyre::Result<()> {
    todo!()
}

#[derive(Serialize, Deserialize)]
enum States {
    New(New),
    Valid(Valid),
    Invalid(Invalid),
    InProgress(InProgress),
    WorkDone(WorkDone),
    WaitingForPayment(WaitingForPayment),
    Paid(Paid),
}

macro_rules! impl_into {
    ($en: ty: $($ty:tt),+) => {
        $(
            impl From<$ty> for $en {
                fn from(t: $ty) -> $en {
                    <$en>::$ty(t)
                }
            }
        )+
    }
}

impl_into!(
    States: New,
    Valid,
    Invalid,
    InProgress,
    WorkDone,
    WaitingForPayment,
    Paid
);
