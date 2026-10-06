pub fn login(creds: models::Credentials) {
    // authenticate.
    crate::database::get_user();
}

fn logout() {
    // Log user out...
}

pub mod models;
