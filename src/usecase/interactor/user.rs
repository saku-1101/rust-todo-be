use crate::domain::user::User;
use crate::usecase::data_access::user::UserDataAccess;
use crate::usecase::input_boundary::user::UserInputBoundary;

struct input_user {
    userDataAccess: dyn UserDataAccess,
}

impl UserInputBoundary for input_user {
    fn create(&self, user: User) -> Result<(), String> {
        self.userDataAccess.create(user)
    }

    fn update_name(&self, id: String, name: String) -> Result<(), String> {
        self.userDataAccess.update_name(name)
    }

    fn delete(&self, id: String) -> Result<(), String> {
        self.userDataAccess.delete(id)
    }

    fn get_all(&self) -> Result<Vec<User>, String> {
        self.userDataAccess.get_all()
    }

    fn get_by_id(&self, id: String) -> Result<User, String> {
        self.userDataAccess.get_by_id(id)
    }
}