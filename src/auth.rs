pub struct User {
  pub user_id: String,
  pub password: String, // Todo: In a real system, passwords should be hashed and salted
}

pub fn authenticate<'a>(users: &'a [User], user_id: &'a str, password: &'a str) -> Option<&'a User> {
  users.iter().find(|user| user.user_id == user_id && user.password == password)
}

// Todo: In a real system, accsess control should be implemented

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_authenticate() {
    let users = vec![
      User {
        user_id: String::from("user1"),
        password: String::from("password1"),
      },
      User {
        user_id: String::from("user2"),
        password: String::from("password2"),
      },
    ];

    let user = authenticate(&users, "user1", "password1");
    assert!(user.is_some());
    assert_eq!(user.unwrap().user_id, "user1");

    let user = authenticate(&users, "user1", "password2");
    assert!(user.is_none());

    let user = authenticate(&users, "user2", "password1");
    assert!(user.is_none());

    let user = authenticate(&users, "user2", "password2");
    assert!(user.is_some());
    assert_eq!(user.unwrap().user_id, "user2");
  }
}
