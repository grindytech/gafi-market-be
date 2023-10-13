use mongodb::bson::{to_bson, Bson, Decimal128};
use std::str::FromStr;

fn decimal128_to_number(decimal: Decimal128, chain_decimal: i32) -> String {
	let string_value = decimal.to_string();
	let right_len = (string_value.chars().count() as i32) - chain_decimal;

	if let Ok(parsed_number) = string_value.parse::<f64>() {
		// Convert the scientific notation to the desired number
		let converted_number = parsed_number * 10_f64.powi(chain_decimal); // 10^9 for E-9

		converted_number.to_string()
	} else {
		"failed".to_string()
	}
}
#[tokio::test]
async fn test_veri() {
	// Simulate a Decimal128 value from MongoDB
	let decimal128_from_mongodb = Decimal128::from_str("2.0E-9").unwrap();
	let value = decimal128_to_number(decimal128_from_mongodb, 10);
	// Convert the Decimal128 to a number
	println!("The value {:?}", value);
}
