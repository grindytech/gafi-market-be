use mongodb::{bson::doc, options::UpdateOptions, results::UpdateResult, Database};
use shared::{types::Result, BaseDocument, NFTOwner, NFT};
use subxt::utils::AccountId32;

use crate::{gafi, workers::RpcClient};

pub async fn refresh_supply(
	collection_id: u32,
	token_id: u32,
	db: &Database,
	api: &RpcClient,
) -> Result<UpdateResult> {
	let query_address = gafi::storage().game().supply_of(collection_id, token_id);
	let supply = api
		.storage()
		.at_latest()
		.await?
		.fetch(&query_address)
		.await?
		.expect(format!("Fail to get supply of {}, {}", collection_id, token_id,).as_str());
	let nft_db = db.collection::<NFT>(NFT::name().as_str());
	let query = doc! {
		"collection_id": collection_id.to_string(),
		"token_id": token_id.to_string(),
	};
	let update = doc! {"$set":{"supply": supply}};
	let rs = nft_db.update_one(query, update.clone(), None).await?;
	log::info!("Nft supply updated {:?}", update);
	Ok(rs)
}

pub async fn refresh_balance(
	target: AccountId32,
	collection_id: String,
	token_id: String,
	db: &Database,
	api: &RpcClient,
) -> Result<UpdateResult> {
	let query_address = gafi::storage().game().item_balance_of(
		target.clone(),
		collection_id.parse::<u32>().unwrap(),
		token_id.parse::<u32>().unwrap(),
	);
	let owner = hex::encode(target.0);
	let balance = api.storage().at_latest().await?.fetch(&query_address).await?.expect(
		format!(
			"Fail to get balance of {}, {}, {}",
			owner.clone(),
			collection_id,
			token_id,
		)
		.as_str(),
	);

	let options = UpdateOptions::builder().upsert(true).build();
	let query = doc! {
		"collection_id":collection_id.clone(),
		"token_id": token_id.clone(),
		"address": owner.clone(),
	};
	let upsert = doc! {"$set":{
		"collection_id":collection_id.clone(),
		"token_id": token_id.clone(),
		"address": owner.clone(),
		"amount": balance
	}};

	let nft_owner_db = db.collection::<NFTOwner>(NFTOwner::name().as_str());
	let rs = nft_owner_db.update_one(query, upsert.clone(), options).await?;
	log::info!("Nft owner updated {:?}", upsert);

	Ok(rs)
}
