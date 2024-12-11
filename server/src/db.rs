use crate::model::{Conversation, User};
use mongodb::bson::{doc, Bson};
use mongodb::{Client, Collection, Database};
use uuid::Uuid;

// let's get reference to the collections. Here' users in the db we have.

pub async fn get_user_collection(client: &Client) -> Collection<User> {
    let db: Database = client.database("Fluentia"); //this is your db name.
    db.collection::<User>("users"); //this is what you've named your collections.
}

pub async fn get_conversation_collection(client: &Client) -> Collection<conversation> {
    let db: Database = client.database("Fluentia");
    db.collection::<Conversation>("conversations");
}

// insert a user into the table.
pub async fn create_user(client: &Client, user: User) -> mongodb::error::Result<()> {
    let collection = get_user_collection(client).await;
    collection.insert_one(user, None).await?;
    Ok(());
}
