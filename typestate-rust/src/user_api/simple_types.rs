
    use super::*;
    pub struct User {
        name: Option<Name>,
        email: Email,
        birthday: Option<Birthday>,
        preferences: Preferences,
    }

    fn create_user(user_request: User) -> Result<User, CreateUserError> {
        validate_user(&user_request)?;
        Ok(
            if let Some(existing_user) = find_user(&user_request.email)? {
                let merged = merge_user(user_request, existing_user)?;
                store(merged)?
            } else {
                store(user_request)?
            },
        )
    }

    fn validate_user(user: &User) -> Result<(), CreateUserError> {
        if !user.email.0.contains("@") {
            return Err(CreateUserError);
        }
        Ok(())
    }

    fn find_user(email: &Email) -> Result<Option<User>, DbError> {
        if rand::random() {
            Ok(None)
        } else {
            Ok(Some(User {
                name: None,
                email: email.clone(),
                birthday: None,
                preferences: Preferences {},
            }))
        }
    }

    fn merge_user(new_user: User, existing_user: User) -> Result<User, MergeError> {
        match (&new_user.name, &existing_user.name) {
            (Some(new_name), Some(existing_name)) if new_name != existing_name => {
                return Err(MergeError);
            }
            _ => {}
        }
        Ok(User {
            name: new_user.name.or(existing_user.name),
            birthday: new_user.birthday.or(existing_user.birthday),
            ..new_user
        })
    }

    fn store(user: User) -> Result<User, DbError> {
        Ok(user)
    }
