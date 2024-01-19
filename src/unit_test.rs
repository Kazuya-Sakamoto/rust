struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(a: i32) -> i32 {
    a * 2
}
fn greeting(name: &str) -> String {
    format!("Hello {}-san", name)
}


#[cfg(test)]
// ファイルの中にサブモジュールのテストを書くことができる
mod tests {
    use super::*;
    #[test]
    fn test_compare_area() {
        let r1 = Rectangle {
            width: 10,
            height: 20,
        };
        let r2 = Rectangle {
            width: 5,
            height: 10,
        };
        assert!(r1.compare_area(&r2));
    }
    #[test]
    fn test_compare_area_2() {
        let r1 = Rectangle {
            width: 3,
            height: 3,
        };
        let r2 = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(!(r1.compare_area(&r2)));
    }
    #[test]
    fn test_double_value() {
        assert_eq!(6, double_value(3));
    }
    #[test]
    fn test_greeting() {
        assert_eq!(greeting("Taro"), "Hello Taro-san");
    }
}