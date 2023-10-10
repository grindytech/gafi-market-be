# Gafi Marketplace

## Table of Contents
- [Description](#description)
- [Installation](#installation)
  - [Rust](#rust)
  - [MongoDB](#mongodb)
- [API Usage](#api-usage)
- [Setup](#setup)

## Description
Gafi Marketplace is a Rust application that serves as the backend for querying data from the Gafi blockchain to be used in a marketplace. The project is built using Rust, a modern system programming language focused on performance, reliability, and productivity.

## Installation

### Rust
To run this project, you will need to have Rust installed on your machine. If you don't have Rust installed, you can follow the guide [here](https://www.rust-lang.org/tools/install).

### MongoDB
This project uses MongoDB as its primary database. To install MongoDB on your machine, follow the guide [here](https://docs.mongodb.com/manual/installation/).

## API Usage
The API provides the following routes:

1. **Get Account** 

    This endpoint retrieves the account details associated with a specific address.

    Route: `/{address}`

    Method: `GET`

2. **Update Favorite**

    This endpoint updates the favorite status of an account.

    Route: `/updateFavorite`

    Method: `POST`

3. **Get Collection**

    This endpoint retrieves a collection based on its ID.

    Route: `/{collection_id}`

    Method: `GET`

4. **Search List Collections**

    This endpoint allows for searching among collections.

    Route: `/search`

    Method: `POST`

## Setup

Once Rust and MongoDB are installed, clone the repository and navigate into the directory:

```bash
git clone https://github.com/grindytech/gafi-market-be.git
cd gafi-market-be
```

Then, build and run the project:

```bash
cargo build
cargo run
```
