use mongodb::bson::oid::ObjectId;

pub struct Forum {
    _id: ObjectId,
    slug: String,
    name: String,
}