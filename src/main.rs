// [dependencies]
// 

use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::collections::HashMap;
use Native_American_Webscraping_Project::Dispenser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let dispenser = Dispenser::new("key.txt", "tribes.txt");

  /*let search = SerpApiSearch::google(params, "secret_api_key".to_string());
  
  let results = search.json().await?;
  let organic_results = results["organic_results"];
  println!("results received");
  println!("--- JSON ---");
  println!(" - results:", organic_results);

  print!("ok"); */
  Ok(())
}