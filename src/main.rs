use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let users_collection = "users";
    let filter = doc! {"name": "Allen Muhani"};

    let new_user = create_document(&users_collection, &filter)
        .await
        .unwrap();

    print!("Inserted user with a user:\n{:#?}", new_user);
    let user = find_one(&users_collection, doc! {"_id": new_user.get("id")}).await?;

    print!("Found a user:\n{:#?}", user);

    Ok(())
}

/**
 * Creates a document in the db given a collection name and document data.
 * @param collection_name the database collection name.
 * @param entry the document to be entered.
 * @returns the a Result with the created document or an error message.
 */
async fn create_document(
    collection_name: &str,
    entry: &Document,
) -> mongodb::error::Result<Document> {
    let collection: Collection<Document> = get_collection(&collection_name).await?;
    let data = collection.insert_one(entry).await?;
    let mut rst = entry.clone();
    rst.insert("id", data.inserted_id);

    Ok(rst)
}
/**
 * Finds one document from a collection.
 * @param collection_name the collection name refference.
 * @param filter the query to run.
 * @return the queried document (if erro or empty should return an empty document)
 */
async fn find_one(collection_name: &str, filter: Document) -> mongodb::error::Result<Document> {
    let collection: Collection<Document> = get_collection(&collection_name).await?;
    let data = collection.find_one(filter).await?.map_or(doc! {}, |x| x);
    Ok(data)
}

/**
 * Gets the collection instance.
 * @param collection_name the collection name.
 * @return the database collection instance.
 */
async fn get_collection(collection_name: &str) -> mongodb::error::Result<Collection<Document>> {
    let uri = "mongodb://localhost:27017";

    let client = Client::with_uri_str(uri).await?;

    let database = client.database(collection_name);

    let users_collection: Collection<Document> = database.collection("users");

    Ok(users_collection)
}
