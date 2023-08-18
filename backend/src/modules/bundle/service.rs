use mongodb::{bson::doc, error, Collection, Database};
use shared::{bundle::Bundle, models};

use crate::common::{Page, QueryPage};

use super::dto::{BundleDTO, QueryFindBundles};

//Find Bundle Detail By id
pub async fn find_bundle_by_id(
	bundle_id: &String,
	db: Database,
) -> Result<Option<BundleDTO>, error::Error> {
	let col: Collection<Bundle> = db.collection(models::bundle::NAME);
	let filter = doc! {"bundle_id":bundle_id};
	if let Ok(Some(bundle_detail)) = col.find_one(filter, None).await {
		Ok(Some(bundle_detail.into()))
	} else {
		Ok(None)
	}
}

/* pub async fn find_bundles(
	params: QueryPage<QueryFindBundles>,
) -> Result<Option<Page<BundleDTO>>, mongodb::error::Error> {
} */
