use std::marker::PhantomData;

struct StateLoggedIn;
struct StateLoggedOut;
struct User<'a, T> {
    name: &'a str,
    password: &'a str,
    _phantom: PhantomData<T>,
}

impl<'a, T> User<'a, T> {
    fn new(name: &'a str, password: &'a str) -> Self {
        User {
            name,
            password,
            _phantom: PhantomData,
        }
    }
}

trait UserRepository {
    fn authenticate<'a>(&self, user: User<'a, StateLoggedOut>) -> User<'a, StateLoggedIn> {
        println!("User atuhenticate!");
        User::new(user.name, user.password)
    }
    fn create(&self, user: User<StateLoggedOut>) {
        println!("Create user!")
    }

    fn delete(&self, user: User<StateLoggedIn>) {
        println!("Delete user!")
    }
}

trait UserRepositoryComponent {
    type UserRepository: UserRepository;
    fn user_repo(&self) -> &Self::UserRepository;
}

trait UserService: UserRepositoryComponent {
    fn authenticate<'a>(&self, username: &'a str, password: &'a str) -> User<'a, StateLoggedIn> {
        self.user_repo().authenticate(User::new(username, password))
    }
}

impl<T: UserRepositoryComponent> UserService for T {}

trait UserServiceComponent {
    type UserService: UserService;
    fn user_service(&self) -> &Self::UserService;
}

struct MockUserRepository;
impl UserRepository for MockUserRepository {}

struct Server {
    user_repo: MockUserRepository,
}
impl UserRepositoryComponent for Server {
    type UserRepository = MockUserRepository;

    fn user_repo(&self) -> &Self::UserRepository {
        &self.user_repo
    }
}

impl UserServiceComponent for Server {
    type UserService = Self;

    fn user_service(&self) -> &Self::UserService {
        self
    }
}

impl Server {
    fn create(&self) {
        self.user_service().authenticate("hoge", "fuga");
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{
        StateLoggedIn, StateLoggedOut, User, UserRepository, UserRepositoryComponent, UserService,
    };

    #[test]
    fn test_auth() {
        struct MockUserRepository;
        impl UserRepository for MockUserRepository {
            fn authenticate<'a>(&self, user: User<'a, StateLoggedOut>) -> User<'a, StateLoggedIn> {
                println!("This is test!!!!!!!!!!!!!");
                User::new("Mock", "User")
            }
        }
        struct MockUserComponent;
        impl UserRepositoryComponent for MockUserComponent {
            type UserRepository = MockUserRepository;

            fn user_repo(&self) -> &Self::UserRepository {
                &MockUserRepository
            }
        }
        let a = MockUserComponent;
        let authed_user = a.authenticate("", "");

        assert_eq!(authed_user.name, "Mock");
        assert_eq!(authed_user.password, "User");
    }
}
