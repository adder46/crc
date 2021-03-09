#[derive(Debug, PartialEq)]
struct DivisionResult {
    quotient: u16,
    remainder: u16,
}

impl DivisionResult {
    fn new(quotient: u16, remainder: u16) -> DivisionResult {
        DivisionResult {
            quotient,
            remainder,
        }
    }
}

#[allow(dead_code)]
fn polynomial_long_division(dividend: u16, divisor: u16) -> DivisionResult {
    let mut quotient = 0;
    let mut remainder = 0;
    let mut tmp = 0;
    for index in 1..length(dividend) + 1 {
        let digit = bring_down(dividend, index);
        tmp <<= 1;
        tmp |= digit;
        if length(divisor) <= length(tmp) {
            quotient <<= 1;
            quotient |= 1;
            tmp ^= divisor;
            if index == length(dividend) {
                remainder = tmp;
            }
        } else {
            quotient <<= 1;
            quotient |= 0;
        }
    }
    DivisionResult::new(quotient, remainder)
}

#[allow(dead_code)]
fn length(n: u16) -> u16 {
    (n as f32).log2() as u16 + 1
}

#[allow(dead_code)]
fn bring_down(number: u16, index: u16) -> u16 {
    number >> (length(number) - index) & 1
}

#[allow(dead_code)]
fn extend_message(message: u16, degree: u16) -> u16 {
    message << degree
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn crc_ok() {
        /* If we think of an (n + 1)-bit message as an n-degree polynomial,
         * we can look at the sender and receiver actually exchanging polynomials
         * with each other. For example, an 8-bit message 10011010 corresponds to
         * the polynomial:
         *
         *   1 * x^7 + 0 * x^6 + 0 * x^5 + 1 * x^4 + 1 * x^3 + 0 * x^2 + 1 * x^1 + 0 * x^0
         *
         *  = x^7 + x^4 + x^3 + x
         *
         * Then the sender and receiver have to agree on a divisor polynomial,
         * for example: x^3 + x^2 + 1 (1101).
         *
         * The sender then multiplies the message with by x^k (where k is the degree of
         * the divisor, in this case 3), which corresponds to extending the message with
         * three zeroes at the end (shifting left by 3). Using binary polynomial long division
         * (where subtraction corresponds to the XOR operation) to find the remainder, we
         * subtracts the remainder (again, XOR) from the message.
         * The message is then sent (10011010101).
         *
         * On the receiving side, the receiver divides the received message by the divisor
         * that was previously agreed upon. The error did not occur during the transmission
         * if it divides cleanly. If the remainder is nonzero, it may be necessary to discard
         * the corrupted message.
         */
        let raw_message = 0b10011010;
        let extended_message = super::extend_message(raw_message, 3);
        let divisor: u16 = 0b1101;
        let rem = super::polynomial_long_division(extended_message, divisor).remainder;
        let resulting_message = extended_message ^ rem;
        assert_eq!(
            super::polynomial_long_division(resulting_message, divisor).remainder,
            0
        );
    }

    #[test]
    fn polynomial_long_division() {
        assert_eq!(
            super::polynomial_long_division(0b10011010000, 0b1101),
            super::DivisionResult::new(0b11111001, 0b101)
        );
    }

    #[rstest(
        input,
        expected,
        case(0b1, 1),
        case(0b10, 2),
        case(0b101, 3),
        case(0b1010, 4),
        case(0b10101, 5),
        case(0b101010, 6),
        case(0b1010101, 7),
        case(0b10101010, 8)
    )]
    fn length(input: u16, expected: u16) {
        assert_eq!(super::length(input), expected);
    }

    #[rstest(index, expected, case(1, 1), case(2, 1), case(3, 0), case(4, 1))]
    fn bring_down(index: u16, expected: u16) {
        let dividend: u16 = 0b1101;
        assert_eq!(super::bring_down(dividend, index), expected);
    }

    #[rstest(
        message,
        degree,
        expected,
        case(0b1010, 2, 0b101000),
        case(0b10011010, 3, 0b10011010000),
        case(0b10101, 4, 0b101010000)
    )]
    fn extend_message(message: u16, degree: u16, expected: u16) {
        assert_eq!(super::extend_message(message, degree), expected);
    }
}
