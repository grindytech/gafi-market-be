use mongodb::Database;
use subxt::{events::EventDetails, OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod gafi {}

mod workers;

pub static mut DB: Option<Database> = None;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Create a client to use:
	let api =
		OnlineClient::<PolkadotConfig>::from_url("wss://rpc-testnet.gafi.network:443").await?;

	// Get events for the latest block:
	let events = api.events().at_latest().await?;

	// We can dynamically decode events:
	println!("Dynamic event details:");
	for event in events.iter() {
		let event = event?;

		let pallet = event.pallet_name();
		let variant = event.variant_name();
		let field_values = event.field_values()?;

		println!("{pallet}::{variant}: {field_values}");
	}

	// Or we can attempt to statically decode them into the root Event type:
	println!("Static event details:");
	for event in events.iter() {
		let event = event?;

		if let Ok(ev) = event.as_root_event::<gafi::Event>() {
			println!("{ev:?}");
		} else {
			println!("<Cannot decode event>");
		}
	}

	for e in events.iter() {
		println!(
			" event name: {:?}",
			e.as_ref().unwrap().event_metadata().variant.name
		);
		println!(
			" pallet name: {:?}",
			e.as_ref().unwrap().event_metadata().pallet.name()
		);
		let newSeed = e
			.and_then(|ev: EventDetails<PolkadotConfig>| {
				ev.as_event::<gafi::game_randomness::events::NewSeed>().map_err(Into::into)
			})
			.transpose();
		if let Some(s) = newSeed {
			match s {
				Ok(seed) => {
					println!(" seed block: {:?}", seed.block_number);
					println!(" seed seed: {:?}", seed.seed);
				},
				Err(err) => {
					println!(" err: {:?}", err);
				},
			}
		}
	}
	// Or we can look for specific events which match our statically defined ones:
	let transfer_event = events.find_first::<gafi::balances::events::Transfer>()?;
	if let Some(ev) = transfer_event {
		println!("  - Balance transfer success: value: {:?}", ev.amount);
	} else {
		println!("  - No balance transfer event found in this block");
	}

	Ok(())
}
