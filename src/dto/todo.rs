use chrono::prelude::*;
use uuid::Uuid;

pub struct Todo {
  id: Option<Uuid>,
  name: String,
  description: String,
  completed: bool,
  author: User,
  created_on: DateTime<Local>,
  last_updated_on: DateTime<Local>,
}

impl Todo {
  pub fn new(
    id: Option<Uuid>,
    name: String,
    description: String,
    completed: bool,
    author: User,
    created_on: DateTime,
    last_updated_on: DateTime,
  ) -> Self {
    Todo {
      id,
      name,
      description,
      completed,
      author,
      created_on,
      last_updated_on,
    }
  }

  pub fn get_id(&self) -> Option<Uuid> {
    self.id
  }

  pub fn get_name(&self) -> String {
    self.name
  }

  pub fn get_description(&self) -> String {
    self.description
  }

  pub fn is_completed(&self) -> bool {
    self.completed
  }

  pub fn get_author(&self) -> User {
    self.author
  }

  pub fn get_created_on(&self) -> DateTime<Local> {
    self.created_on
  }

  pub fn get_last_updated_on(&self) -> DateTime<Local> {
    self.last_updated_on
  }
}
