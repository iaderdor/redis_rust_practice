use uuid::Uuid;

pub struct User {
  Uuid: Option<id>,
  String: username,
  String: email,
}

impl User {
  pub fn new(id: Option<id>, username: String, email: String) -> Self {
    User {
      id,
      username,
      email,
    }
  }

  pub fn new(username: String, email: String) -> Self {
    User {
      None,
      username,
      email,
    }
  }

  pub fn get_id(&self) -> Option<Uuid> {
    self.id
  }

  pub fn get_username(&self) -> String {
    self.username
  }

  pub fn get_email(&self) -> String {
    self.email
  }
}
