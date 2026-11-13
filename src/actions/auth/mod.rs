pub mod forgot_password;
pub mod login;
pub mod register;
pub mod reset_password;
pub mod roles;
pub mod update_user_information;
pub mod update_user_password;
pub mod verify_email;

pub fn is_alpha_dash(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
