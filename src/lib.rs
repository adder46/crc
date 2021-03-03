#[allow(dead_code)]
fn polynomial_long_division(dividend: u16, divisor: u16) -> (u16, u16) {
    let mut result = 0;
    let mut remainder = 0;
    let mut tmp = 0;
    for index in 1..length(dividend) + 1 {
        let digit = bring_down(dividend, index);
        tmp <<= 1;
        tmp |= digit;
        if length(divisor) <= length(tmp) {
            result <<= 1;
            result |= 1;
            tmp = tmp ^ divisor;
            if index == length(dividend) {
                remainder = tmp; 
            }
        } else {
            result <<= 1;
            result |= 0;
        }
    }
    (result, remainder)
}

#[allow(dead_code)]
fn length(n: u16) -> u16 {
    (n as f32).log2() as u16 + 1
}

#[allow(dead_code)]
fn bring_down(number: u16, index: u16) -> u16 {
    (number >> length(number) - index) & 1 as u16
}

#[allow(dead_code)]
fn extend_message(message: u16, degree: u16) -> u16 {
    message << degree
}

#[cfg(test)]
mod tests {

    #[test]
    fn crc() {
        let message = 0b10011010;
        let extended_message = super::extend_message(message, 3);
        let divisor: u16 = 0b1101;
        let rem = super::polynomial_long_division(extended_message, divisor).1;
        let recovered_message = extended_message ^ rem;
        assert_eq!(super::polynomial_long_division(recovered_message, divisor).1, 0);
    }
    
    #[test]
    fn bring_down() {
        let dividend: u16 = 0b1101;
        assert_eq!(super::bring_down(dividend, 1), 1);
        assert_eq!(super::bring_down(dividend, 2), 1);
        assert_eq!(super::bring_down(dividend, 3), 0);
        assert_eq!(super::bring_down(dividend, 4), 1);
    }
    #[test]
    fn it_works() {
        assert_eq!(super::polynomial_long_division(0b10011010000, 0b1101), (0b11111001, 0b101));
    }
}