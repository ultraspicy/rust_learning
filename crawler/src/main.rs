use tokio_postgres::{NoTls, Error};
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Connecting to database...");

    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect(
        "host=34.60.219.255 user=postgres password=002309Gjf! dbname=test_db",
        NoTls,
    ).await?;

    println!("Connected!");

    // Spawn connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Random sentences
    let sentences = vec![
        "The quick brown fox jumps over the lazy dog",
        "Rust is a systems programming language",
        "PostgreSQL is a powerful database",
        "Cloud SQL makes database management easy",
        "Web crawlers collect data from websites",
    ];

    // Pick a random sentence
    let mut rng = rand::thread_rng();
    let random_sentence = sentences[rng.gen_range(0..sentences.len())];

    println!("Inserting: {}", random_sentence);

    // Insert into database
    client.execute(
        "INSERT INTO dummy (data) VALUES ($1)",
        &[&random_sentence],
    ).await?;

    println!("Successfully inserted: {}", random_sentence);

    Ok(())
}
