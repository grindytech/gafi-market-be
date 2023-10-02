use std::collections::HashMap;

use mongodb::bson::{Bson, Document};
use serde_json::{Map, Value};

use crate::types;

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

pub fn vec_to_array(vec: Vec<u8>) -> [u8; 32] {
	let mut arr_u8: [u8; 32] = [0; 32];
	for i in 0..32 {
		arr_u8[i] = *vec.get(i).unwrap_or(&0u8);
	}
	arr_u8
}

pub fn serde_json_to_doc(data: Value) -> types::Result<(Document, Map<String, Value>)> {
	let obj: Map<String, Value> = data.as_object().ok_or("Not an object")?.clone();

	let mut attributes_map: HashMap<String, Bson> = HashMap::new();
	obj.clone().into_iter().for_each(|(key, val)| {
		attributes_map.insert(key, Bson::String(val.to_string()));
	});
	let attributes: Document = attributes_map.into_iter().collect();
	Ok((attributes, obj))
}
