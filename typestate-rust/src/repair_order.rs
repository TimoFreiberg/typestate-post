pub mod kitchen_sink;

// TODO add data
#[derive(Debug)]
pub struct Customer;

impl Customer {
    pub fn has_outstanding_debt(&self) -> bool {
        // TODO ask the database
        false
    }
    pub fn is_banned(&self) -> bool {
        // TODO ask the database
        false
    }
}

#[derive(Debug)]
pub struct Employee;
