struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let validator =
        |username: &str, password: &str| -> bool { !username.is_empty() && !password.is_empty() };

    let weak_password = "password123!".to_owned();
    // Fn - Immutably borrow variable in environment.
    // FnMut - Mutably borrow variables in environment. Can change environment.
    // FnOnce - Take Ownership of variables in environment. Can only be called once.// every closure implement this. In order to take an ownerships, we should write `move` keyword in front of double pipeline which indicates implementation of FnOnce -> that means taking ownerships of variable.
    // NOTE: closure are anynomous function that are store in variable and can be pass as an argument.
    let validator2 = |username: &str, password: &str| -> bool {
        !username.is_empty()
            && !password.is_empty()
            && password.len() > 8
            && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
            && password != weak_password
    };

    println!("weak_password: {weak_password}");

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123^".to_owned(),
        validator: validator2,
    };

    // println!("{}", validate_credentials(&creds.username, &creds.password));
    // println!("{}", validator(&creds.username, &creds.password));
    println!("{}", creds.is_valid());
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}
