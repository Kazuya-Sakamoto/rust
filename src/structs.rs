#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Self {
            width,
            height,
        }
    }
    fn area(&self) { // 参照にすることで所有権を奪わない
        println!("{}", self.width * self.height);
    }
    
}

pub fn run() {
    // let user1 = User {
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("antoheremail@example.com");
    println!("{:#?}", user1);
    let user2 = build_user(String::from("user2@example"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, 
        username,
        active: true,
        sign_in_count: 1,
    }
}