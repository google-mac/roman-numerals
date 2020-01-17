// code concepts from https://www.geeksforgeeks.org/converting-decimal-number-lying-between-1-to-3999-to-roman-numerals/
pub fn number_to_numeral(mut number: u32) -> Result<String, &'static str> {
    // TODO: if number is greater than 3999 or less than 1, throw error that it can't work
    if number < 1 {
        return Err("Number cannot be less than 1.");
    }
    if number > 3999 {
        return Err("Number cannot be greater than than 3999.");
    }

    let groups = [1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000];
    let symbols = [
        "I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M",
    ];

    let mut result: Vec<&str> = vec![];
    let mut index: usize = 12;
    let mut dividend;

    while index >= 0 {
        dividend = number / groups[index];
        number %= groups[index];

        while dividend > 0 {
            result.push(symbols[index]);
            dividend -= 1;
        }

        if index == 0 {
            break;
        }

        index -= 1;
    }

    Ok(result.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_generate_3() {
        assert_eq!(number_to_numeral(3).unwrap(), "III");
    }

    #[test]
    fn it_can_generate_32() {
        assert_eq!(number_to_numeral(32).unwrap(), "XXXII");
    }

    #[test]
    fn it_can_generate_42() {
        assert_eq!(number_to_numeral(42).unwrap(), "XLII");
    }

    #[test]
    fn it_can_generate_191() {
        assert_eq!(number_to_numeral(191).unwrap(), "CXCI");
    }

    #[test]
    fn it_can_generate_2387() {
        assert_eq!(number_to_numeral(2387).unwrap(), "MMCCCLXXXVII");
    }

    #[test]
    fn it_errors_when_number_is_below_1() {
        assert!(number_to_numeral(0).is_err());
    }

    #[test]
    fn it_errors_when_number_is_above_3999() {
        assert!(number_to_numeral(4000).is_err());
    }
}
