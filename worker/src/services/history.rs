use mongodb::{
	bson::{doc, Document},
	options::UpdateOptions,
	Database,
};
use shared::{types::Result, BaseDocument, HistoryTx};

pub async fn upsert(history: HistoryTx, db: &Database) -> Result<()> {
	let history_db: mongodb::Collection<HistoryTx> =
		db.collection::<HistoryTx>(HistoryTx::name().as_str());
	let query = doc! {
	  "extrinsic_index": history.extrinsic_index,
	  "event_index": history.event_index,
	  "block_height": history.block_height,
	};
	let upsert: Document = history.into();
	let options = UpdateOptions::builder().upsert(true).build();
	history_db.update_one(query, upsert, options).await?;
	Ok(())
}
