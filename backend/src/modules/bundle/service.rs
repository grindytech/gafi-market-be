use actix_web::http;
use mongodb::{
	bson::{doc, oid::ObjectId},
	error, Collection, Database,
};
use shared::{bundle::Bundle, models};

use crate::common::{ErrorResponse, Page, QueryPage};

use super::dto::BundleDTO;

//Find Bundle Detail By id
pub async fn find_bundle_by_id(
	bundle_id: &String,
	db: Database,
) -> Result<Option<BundleDTO>, error::Error> {
	let col: Collection<Bundle> = db.collection(models::bundle::NAME);

	let filter = doc! {"bundle_id":bundle_id};
	/*
	//Catch Error
	let bundle_detail = col.find_one(filter, None).await;
	match bundle_detail {
		Ok(Some(bundle_detail_doc)) => Ok(Some(bundle_detail_doc.into())),
		Ok(None) => Ok(None),
		Err(e) => Err(e),
	} */
	if let Ok(Some(bundle_detail)) = col.find_one(filter, None).await {
		Ok(Some(bundle_detail.into()))
	} else {
		Ok(None)
	}
}
