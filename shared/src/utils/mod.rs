pub fn string_decimal_to_number(str: &str, decimal: i32) -> String {
	let left_len = (str.chars().count() as i32) - decimal;
	let left: String;
	let right: String;

	if left_len == 0 {
		left = "0".to_string();
		right = str.to_string()
	} else if left_len > 0 {
		left = str.chars().take(left_len as usize).collect();
		right = str.chars().skip(left_len as usize).collect();
	} else {
		let right_zeros = (left_len..0).map(|_| "0").collect::<String>();
		right = right_zeros + str;
		left = "0".to_string();
	}
	format!("{}.{}", left, right)
}

#[test]
fn test() {
	assert_eq!(string_decimal_to_number("123456789", 3), "123456.789");
	assert_eq!(string_decimal_to_number("123456789", 9), "0.123456789");
	assert_eq!(string_decimal_to_number("123456789", 12), "0.000123456789");
}
