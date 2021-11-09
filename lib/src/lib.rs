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
    fn it_adds_1_and_3() {
        let result = Calculator::add(1, 3);
        assert_eq!(result, 4);
    }


    #[test]
    fn it_adds_2_and_2() {
        assert_eq!(4, Calculator::add(2, 2));
    }
}
