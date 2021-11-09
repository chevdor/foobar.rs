pub struct Calculator;

impl Calculator {
    pub fn add(a : u8, b: u8) -> u8 {
        return a + b;
    }
}


#[cfg(test)]
mod tests {
    use crate::Calculator;

    #[test]
    fn it_works() {
        let result = Calculator::add(1, 3);
        assert_eq!(result, 4);
    }
}
