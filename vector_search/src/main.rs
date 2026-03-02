use std::vec;
use qdrant_client::{
    Payload, Qdrant, QdrantError,
    qdrant::{
        Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, QueryPointsBuilder,
        ScalarQuantizationBuilder, SearchParamsBuilder, UpsertPointsBuilder,
        VectorParamsBuilder,
    },
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), QdrantError> {
    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let collection_list = client.list_collections().await?;
    dbg!(collection_list);

    let collection_name = "test";
    let init = client.delete_collection(collection_name).await?;
    dbg!(init);

    client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine))
                .quantization_config(ScalarQuantizationBuilder::default()),
        )
        .await?;

    let collection_info = client.collection_info(collection_name).await?;
    dbg!(client.list_collections().await?);
    dbg!(collection_info);

    let payload: Payload = serde_json::json!(
        {
            "foo": "bar",
            "bar": 12
        }
    )
    .try_into()?;

    let point = vec![PointStruct::new(Uuid::new_v4().to_string(), vec![9.99; 10], payload)];
    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, point))
        .await?;

    let query_result = client
        .query(
            QueryPointsBuilder::new(collection_name)
                .query(vec![10.0; 10])
                .limit(10)
                .filter(Filter::all([Condition::matches("bar", 12)]))
                .with_payload(true)
                .params(SearchParamsBuilder::default().exact(true)),
        )
        .await?;
    dbg!(&query_result);

    if let Some(first_point) = query_result.result.first() {
        dbg!(first_point);
    }

    Ok(())
}
