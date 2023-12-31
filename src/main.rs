use anyhow::{anyhow, Result};
use dotenvy::dotenv;
use log::info;
use meilisearch_sdk::Client;

use google_sheets4::{client::NoToken, hyper, hyper_rustls, Sheets};
use sheet_to_meilisearch::Entry;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let auth = std::env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable must be defined");
    info!("Auth Key: {auth}");
    let hub = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        NoToken,
    );

    // Create a client (without sending any request so that can't fail)
    let client = Client::new(
        std::env::var("MEILISEARCH_URL")
            .expect("MEILISEARCH_URL environment variable must be defined"),
        Some(
            std::env::var("MEILISEARCH_API_KEY")
                .expect("MEILISEARCH_API_KEY environment variable must be defined"),
        ),
    );

    let index = client.index(
        std::env::var("MEILISEARCH_INDEX_NAME")
            .expect("MEILISEARCH_INDEX_NAME environment variable must be defined"),
    );

    let spreadsheet_id = std::env::var("SPREADSHEET_ID")
        .expect("SPREADSHEET_ID environment variable must be defined");
    let result = hub
        .spreadsheets()
        .values_get(&spreadsheet_id, "Sheet1!A2:G1000")
        .param("key", &auth)
        .doit()
        .await?;
    let entries = match result.1.values {
        Some(v) => v
            .into_iter()
            .filter_map(|row| {
                let name = row[0].as_str().unwrap().to_string();
                if name.is_empty() {
                    return None;
                }
                let edition = row[1].as_str().and_then(|s| Some(s.to_string()));
                let format = row[2].as_str().and_then(|s| Some(s.to_string()));
                let system = row[3].as_str().and_then(|s| Some(s.to_string()));
                let r#type = row[4].as_str().and_then(|s| Some(s.to_string()));
                let id = safe_string(&Some(name.clone()))
                    + &safe_string(&edition)
                    + &safe_string(&format)
                    + &safe_string(&system);
                Some(Entry {
                    id: id,
                    name,
                    edition,
                    format,
                    system,
                    r#type,
                    pdf: row[5].as_bool().unwrap_or(false),
                    physical: row[6].as_bool().unwrap_or(false),
                })
            })
            .collect::<Vec<Entry>>(),
        None => return Err(anyhow!("No values in Google Sheets response!")),
    };
    info!("Entries to add/update: {}", entries.len());

    let task = index
        .delete_all_documents()
        .await
        .expect("Failed to delete all items");
    client.wait_for_task(task, None, None).await.unwrap();
    index
        .add_documents(&entries, Some("id"))
        .await
        .expect("Failed to update entries");
    Ok(())
}

fn safe_string(str: &Option<String>) -> String {
    if str.is_none() {
        return "".to_owned();
    }
    (str.as_deref())
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c == &'_' || c == &'-')
        .collect()
}
