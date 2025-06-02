

#[allow(dead_code)]
trait Toint {
fn new(string: String) -> Self;
    fn toint(num: String) -> i32;
}

impl Toint for String {

    fn new(string: String) -> String {
        return string;
    }

    fn toint(num: String) -> i32 {
        return num.parse::<i32>().unwrap();
    }
}

