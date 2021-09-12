
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width > other.width
    }
}

// assert_eq!와 assert_ne!를 이용한 동치(equality) 테스트
pub fn add_two(a: i32) -> i32 {
    a + 3
}

// 커스텀 실패 메세지 추가하기
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

//should_panic을 이용한 패닉에 대한 체크
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        } else if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}



#[cfg(test)]
mod tests {
    use core::panic;
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test panic");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 }; 
        
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    // assert_eq!와 assert_ne!를 이용한 동치(equality) 테스트
    #[test]
    fn it_adds_two() {
        assert_eq!(4,add_two(2));
    }

    // 커스텀 실패 메세지 추가하기
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    //should_panic을 이용한 패닉에 대한 체크
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}