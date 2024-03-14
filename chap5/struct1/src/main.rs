fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1.email: {}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("user1.email: {}", user1.email);

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        username: String::from("anotherusername567"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        ..user1 // 使用剩余字段初始化语法从其他实例中获取剩余的值
    };
    println!("user2.active: {}", user2.active);
}

struct User {
    // username: &str, // error[E0106]: missing lifetime specifier
    // email: &str, // error[E0106]: missing lifetime specifier
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }
    // 等价于
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}