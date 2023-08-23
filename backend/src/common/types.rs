use mongodb::bson::Document;

pub trait DBQuery {
	fn to_doc(&self) -> Document;
}
