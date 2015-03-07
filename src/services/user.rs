use entities::user::User;

pub fn create(user: &mut User) {
    // persist user
    user.id = 1000;
}
