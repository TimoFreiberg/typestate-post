use time::Date;

pub mod simple_types;

pub mod duplicated_specific_types {
    use super::Preferences;
    pub struct DbUser {
        row_id: u64,
        name: Option<String>,
        email: String,
        birthday: Option<String>,
        preferences: Preferences,
    }
}

struct Preferences {}
#[derive(Eq, PartialEq)]
struct Name(String);
#[derive(Clone)]
struct Email(String);
struct Birthday(Date);

struct CreateUserError;

impl From<DbError> for CreateUserError {
    fn from(_: DbError) -> Self {
        CreateUserError
    }
}

impl From<MergeError> for CreateUserError {
    fn from(_: MergeError) -> Self {
        CreateUserError
    }
}

struct DbError;

struct MergeError;
