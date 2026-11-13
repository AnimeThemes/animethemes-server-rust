pub mod forgot_password;
pub mod login;
pub mod register;
pub mod reset_password;
pub mod roles;
pub mod update_user_information;
pub mod update_user_password;
pub mod verify_email;

pub fn is_alpha_dash(value: &str) -> bool {
    regex::Regex::new(r"^[\p{L}\p{M}\p{N}_-]+$")
        .unwrap()
        .is_match(value)
}
