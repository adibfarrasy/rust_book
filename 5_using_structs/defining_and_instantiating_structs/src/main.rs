fn main() {
    let mut _user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    _user1.email = String::from("anotheremail@exaple.com");

    let _user2 = User {
        email: String::from("another@example.com"),
        .._user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// using struct shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;
