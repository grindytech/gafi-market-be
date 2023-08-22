use std::collections::HashMap;

use mongodb::bson::{doc, Bson, Document};
use serde_json::Value;

pub async fn create_or_query(criteria: HashMap<String, Value>) -> Document {
	let mut or_conditions = vec![];

	for (field, value) in criteria {
		if value != Value::Null {
			let bson_value = match value {
				Value::Bool(v) => Bson::Boolean(v),
				Value::Number(n) => Bson::Double(n.as_f64().unwrap_or_default()), // Convert to double
				Value::String(s) => Bson::String(s),
				// Handle other JSON types as needed
				_ => continue,
			};
			or_conditions.push(doc! {field: bson_value});
		}
	}

	if !or_conditions.is_empty() {
		doc! {"$or": or_conditions}
	} else {
		Document::new()
	}
}

pub async fn get_total_page(number_items: usize, size: u64) -> u64 {
	(number_items as f64 / size as f64).ceil() as u64
}

pub async fn get_filter_option(
	order_by: String,
	des: bool,
) -> Option<mongodb::options::FindOptions> {
	let sort = if des { 1 } else { -1 };
	let sort = doc! { order_by:sort };
	let mut find_options = mongodb::options::FindOptions::default();
	find_options.sort = Some(sort);
	Some(find_options)
}
