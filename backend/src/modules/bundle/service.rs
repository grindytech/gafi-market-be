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
	if let Ok(Some(bundle_detail)) = col.find_one(filter, None).await {
		log::info!("Await >> {:?}", bundle_detail);
		Ok(Some(bundle_detail.into()))
	} else {
		Ok(None)
	}
}

/* pub async fn create_bundles(bundle: BundleDTO, db: Database) -> Result<String, ErrorResponse> {
	let col: Collection<Bundle> = db.collection(models::bundle::NAME);
	let entity: Bundle = Bundle {
		id: Some(ObjectId::new()),
		bundle_id: bundle.bundle_id,
		creator: bundle.creator,
		name: bundle.name,
		description: bundle.description,
		items: bundle.items.iter().map(|value| value.clone().into()).collect(),
		market_type: bundle.market_type,
		status: bundle.status,
		price: bundle.price,
		begin_at: bundle.begin_at,
		end_at: bundle.end_at,
		update_at: bundle.update_at,
		create_at: bundle.create_at,
	};
	let rs = col.insert_one(entity.clone(), None).await;
	match rs {
		Ok(r) => Ok(r.inserted_id.to_string()),
		Err(e) => Err(ErrorResponse {
			message: e.to_string(),
			status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16().to_string(),
		}),
	}
}
 */
