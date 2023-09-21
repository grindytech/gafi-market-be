use mongodb::{
	bson::{doc, Document},
	options::UpdateOptions,
	results::UpdateResult,
	Database,
};
use shared::{BaseDocument, HistoryTx};

pub async fn upsert(
	history: HistoryTx,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let history_db: mongodb::Collection<HistoryTx> =
		db.collection::<HistoryTx>(HistoryTx::name().as_str());
	let query = doc! {
	  "extrinsic_index": history.extrinsic_index,
	  "event_index": history.event_index,
	  "block_height": history.block_height,
	};
	let history_doc: Document = history.into();
	let upsert = doc! { "$set": history_doc };
	let options = UpdateOptions::builder().upsert(true).build();
	let rs = history_db.update_one(query, upsert, options).await?;
	Ok(rs)
}
