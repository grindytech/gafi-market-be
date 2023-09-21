pub use shared::types::Result;

use crate::{
	gafi::{self, game::events::CollectionCreated},
	services,
	workers::{HandleParams, EventHandle},
};

use shared::constant::{EVENT_COLLECTION_CREATED, EVENT_COLLECTION_METADATA_SET};

/// - Handles the creation of an NFT collection.
/// - Updates the MongoDB collection with the new collection data and logs the event.
async fn on_collection_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<CollectionCreated>()?;
	if let Some(collection) = event_parse {
		services::nft_collection::upsert_without_metadata(
			&collection.collection.to_string(),
			&hex::encode(collection.who.0),
			params.db,
		)
		.await?;
	};
	Ok(())
}

async fn on_collection_metadata_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::nfts::events::CollectionMetadataSet>()?;
	if let Some(ev) = event_parse {
		let data = String::from_utf8(ev.data.0).ok();
		if let Some(metadata) = data {
			services::nft_collection::update_collection_metadata(metadata, ev.collection, params.db)
				.await?;
		}
	};
	Ok(())
}

pub fn tasks() -> Vec<EventHandle> {
	vec![
		EventHandle::new(EVENT_COLLECTION_CREATED, move |params| {
			Box::pin(on_collection_created(params))
		}),
		EventHandle::new(EVENT_COLLECTION_METADATA_SET, move |params| {
			Box::pin(on_collection_metadata_set(params))
		}),
	]
}
