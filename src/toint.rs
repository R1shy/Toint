
#[allow(dead_code)]
pub trait ToInt {
    fn to_int(&self) -> Result<i32, std::num::ParseIntError>;
}

impl ToInt for &str {
    fn to_int(&self) -> Result<i32, std::num::ParseIntError> {
        self.parse::<i32>()
    }
}

impl ToInt for String {
    fn to_int(&self) -> Result<i32, std::num::ParseIntError> {
        self.parse::<i32>()
    }
}

