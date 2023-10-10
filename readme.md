# GAFI Marketplace Backend

## Description

The GAFI Marketplace Backend is a backend project written in Rust that provides APIs for the GAFI Chain marketplace. It uses actix_web framework to handle HTTP requests and interacts with a MongoDB database. The project is organized as a cargo workspace with three main packages: `backend`, `worker`, and `shared`.

## Features

- RESTful APIs for the GAFI Chain marketplace
- Real-time event processing using the `worker` package

## Installation and Setup

To install and set up the project locally, follow these steps:

1. Ensure you have Rust and Cargo installed on your machine. If not, you can install them from [here](https://www.rust-lang.org/tools/install).

2. Clone the repository:

   ```shell
   git clone https://github.com/grindytech/gafi-market-be.git
   ```

3. Navigate to the project directory:

   ```shell
   cd gafi-market-be
   ```

4. Install the necessary dependencies:

   ```shell
   cargo build
   ```

5. Set up the environment variables by creating a `.env` file in the project root directory. Use the provided `.env.example` file as a template and fill in the required values.

6. Start the backend server:

   ```shell
   cargo run -p backend
   ```

   The server should now be running on `http://localhost:8080`.

7. Start the worker
   ```shell
   cargo run -p worker
   ```

7. (Optional) We use `mongodb-memory-server` to unit test, so if you want to run unit tests, you need to install Node.js and the required dependencies. Run the following commands:

   ```shell
   npm install
   ```

   After that, you can run the unit tests using the following command:

   ```shell
   cargo test
   ```

## Project Structure

The project is organized as follows:

- `backend`: Contains the code for the RESTful APIs using the actix_web framework.
- `worker`: Listens for new blocks and processes events.
- `shared`: Contains common code such as models, utils, types, constants, and database connection.

## Configuration

The project uses environment variables for configuration. Create a `.env` file in the project root directory and fill in the required values. The following variables are used:

- `MONGODB_URI`: The URI for connecting to the MongoDB database.
- `MONGODB_DB_NAME`: The name of the MongoDB database.
- `JWT_TOKEN_SECRET`: The secret key used for JWT token generation and validation.
- `JWT_EXPIRE_TIME`: The expiration time for JWT tokens.
- `START_BLOCK`: The block height that worker begin handle.
- `RPC`: The websocket rpc uri.

## Architecture

<div hidden style="visibility:hidden">

```plantuml
@startuml
skinparam linetype ortho
package "Blockchain" {
  "New block" - [Events]
}
node "Worker" {
  [Event handlers]
  Events --> "Event handlers" : Process blockchain Events
  
}

note as EventHandleNote
 mint, transfer, burn, 
 metdata, trade, game,
 collecction, etc..
end note

EventHandleNote .. "Event handlers"

node "Backend" {
	[APIs]
}
HTTP ..> APIs : use RESTful APIs

database "MongoDB" {
	"Event handlers" --> "MongoDB" 
	"MongoDB" --> "Event handlers"
	
	"Backend" --> "MongoDB"
	"MongoDB" --> "Backend"
}

@enduml
```

</div>

![architecture diagram image](img/architecture_diagram.svg)

## Database schema

<div hidden style="visibility:hidden">

```plantuml
@startuml
skinparam linetype ortho
object block 
block : height: u32 
block : hash: String

object game
game : game_id: String
game : owner: String
game : category: Option<String>
game : description: Option<String>
game : logo_url: Option<String>
game : banner_url: Option<String>
game : name: Option<String>
game : collections: Option<Vec<String>>

object nft_collection
nft_collection : collection_id: String
nft_collection : category: Option<String>
nft_collection : owner: String
nft_collection : games: Option<Vec<String>>
nft_collection : name: Option<String>
nft_collection : logo_url: Option<String>
nft_collection : banner_url: Option<String>
nft_collection : external_url: Option<String>

object nft
nft : token_id: String
nft : collection_id: String
nft : supply: Option<u32>
nft : created_by: String
nft : attributes: Option<Vec<Property>>
nft : name: Option<String>
nft : description: Option<String>
nft : external_url: Option<String>
nft : image: Option<String>
nft : animation_url: Option<String>

object property
property : key: string
property : value: json string

object nft_owner
nft_owner : token_id: String
nft_owner : collection_id: String
nft_owner : address: String
nft_owner : amount: i32

object account
account : address: String
account : name: String
account : nonce: Option<String>

object loot_table
loot_table : nft: Option<{collection, token_id as item}>
loot_table : weight: u32

object pool
pool : pool_id: String
pool : owner: String
pool : type_pool: String
pool : mint_type: String
pool : admin: String
pool : minting_fee: Decimal128
pool : begin_at: i64
pool : end_at: i64
pool : loot_table: Vec<LootTable>

object request_mint
request_mint : who: String
request_mint : pool: String
request_mint : target: String
request_mint : block: u32
request_mint : event_index: u32
request_mint : execute_block: u32
request_mint : extrinsic_index: i32

object trade
trade : trade_id: String
trade : trade_type: String
trade : owner: String

trade : start_block: Option<u32>
trade : end_block: Option<u32>
trade : duration: Option<u32> #auction

trade : unit_price: Option<Decimal128> #set buy, set price
trade : maybe_price: Option<Decimal128> #auction, swap
trade : price: Option<Decimal128> #bundle

trade : nft: Option<Nft> #set buy, set price
trade : source: Option<Vec<Nft>> #swap, auction
trade : maybe_required: Option<Vec<Nft>> #swap
trade : bundle: Option<Vec<Nft>> #bundle
trade : wish_list: Option<Vec<Nft>>,
trade : status: String #ForSale, Sold, Canceled, Expired

object history
history : extrinsic_index: i32
history : event_index: u32
history : block_height: u32
history : value: Option<Decimal128>
history : event: String
history : from: String
history : to: Option<String>
history : pool: Option<String>
history : nfts: Option<Vec<Nft>>
history : amount: Option<u32>
history : price: Option<Decimal128>
history : trade_id: Option<String>
history : trade_type: Option<String>
history : source: Option<Vec<Nft>> #swap,

pool  ..{ request_mint 
pool  ..{ loot_table 
nft  ..{ loot_table 
game  }..{ nft_collection 
nft  }.. nft_collection 
nft_owner  }.. nft 
nft_owner  }.. account 
trade }.. nft
history }.. nft
history }.. trade
property }.. nft


@enduml
```

</div>

![database schema](img/db_schema_diagram.svg)

## Contributing

If you would like to contribute to the project, please follow these guidelines:

1. Fork the repository and clone it to your local machine.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them to your branch.
4. Push your branch to your forked repository.
5. Open a pull request to the main repository.

## License

This project is licensed under the [MIT License](LICENSE).

