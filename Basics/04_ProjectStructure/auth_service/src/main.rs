use auth_service::Credentials;

fn main() {
    let cred = Credentials {
        username: "Rusty1".to_owned(),
        password: "password123".to_owned(),
    };

    auth_service::authenticate(cred);
}
