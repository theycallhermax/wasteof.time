pub mod errors {
    mod notfound;
    pub use notfound::NotFound;
}

mod about;
pub use about::About;

mod home;
pub use home::Home;

mod user;
pub use user::User;
