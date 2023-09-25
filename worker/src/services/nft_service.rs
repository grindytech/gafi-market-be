use mongodb::{
	bson::{doc, Bson, DateTime, Document},
	options::UpdateOptions,
	results::UpdateResult,
	Database,
};
use serde::Deserialize;
use shared::{types::Result, BaseDocument, NFTOwner, RequestMint, NFT};
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

pub async fn upsert_request_mint(
	request_mint: RequestMint,
	db: &Database,
) -> std::result::Result<UpdateResult, mongodb::error::Error> {
	let request_doc: Document = request_mint.clone().into();
	let request_mint_db = db.collection::<RequestMint>(RequestMint::name().as_str());
	let query = doc! {
		"block": request_mint.block,
		"event_index": request_mint.event_index,
	};
	let update = doc! {
		"$set": request_doc,
	};
	let options = UpdateOptions::builder().upsert(true).build();
	let rs = request_mint_db.update_one(query, update, options).await?;
	Ok(rs)
}
pub async fn get_rq_mint(
	block: u32,
	event_index: u32,
	db: &Database,
) -> std::result::Result<Option<RequestMint>, mongodb::error::Error> {
	let request_mint_db = db.collection::<RequestMint>(RequestMint::name().as_str());
	let query = doc! {
		"block": block,
		"event_index": event_index,
	};
	let rs = request_mint_db.find_one(query, None).await?;
	Ok(rs)
}
#[derive(Deserialize, Debug)]
struct NftMetadata {
	title: Option<String>,
	image: Option<String>,
}
pub async fn nft_metadata_set(
	metadata: &str,
	collection_id: &str,
	token_id: &str,
	db: &Database,
) -> shared::Result<()> {
	let data = serde_json::from_str::<NftMetadata>(&metadata);
	let update: Document;
	match data {
		Ok(data) => {
			update = doc! {
					"$set": {
					"img_url": data.image,
					"name": data.title,
					"updated_at": DateTime::now(),
					"metadata": metadata,
				}
			};
		},
		Err(_) => {
			update = doc! {
					"$set": {
					"updated_at": DateTime::now(),
					"metadata": metadata,
					"img_url": Bson::Null,
					"name": Bson::Null,
				}
			};
		},
	}
	let nft_db = db.collection::<NFT>(NFT::name().as_str());
	let query = doc! {
		"token_id": token_id.to_string(),
		"collection_id": collection_id.to_string()
	};
	nft_db.update_one(query, update, None).await?;
	Ok(())
}

pub async fn upsert_nft_without_metadata(
	collection_id: &str,
	token_id: &str,
	created_by: &str,
	maybe_supply: Option<u32>,
	db: &Database,
) -> std::result::Result<UpdateResult, mongodb::error::Error> {
	let nft_db = db.collection::<NFT>(&NFT::name());
	let options = UpdateOptions::builder().upsert(true).build();
	let query = doc! {
		"token_id": token_id,
		"collection_id": collection_id,
	};
	let upsert = doc! {"$set": {
			"token_id": token_id,
			"collection_id": collection_id,
			"created_by": created_by,
			"supply": maybe_supply,
			"created_at": DateTime::now(),
			"updated_at": DateTime::now()
		}
	};
	let rs = nft_db.update_one(query, upsert, options).await?;
	Ok(rs)
}

pub async fn get_nft_by_token_id(
	token_id: &str,
	collection_id: &str,
	db: &Database,
) -> std::result::Result<Option<NFT>, mongodb::error::Error> {
	let nft_db = db.collection::<NFT>(&NFT::name());
	let query = doc! {
		"token_id": token_id,
		"collection_id": collection_id,
	};
	let nft = nft_db.find_one(query, None).await;
	nft
}
