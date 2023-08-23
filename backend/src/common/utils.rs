use mongodb::bson::doc;

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
