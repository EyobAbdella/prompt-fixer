use dotenvy::dotenv;
use std::env;
use reqwest::blocking::Client;
use serde_json::json;

fn main(){
    dotenv().ok(); 


    let API_KEY = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");


    // let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent"; 
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-lite:generateContent";

    let client = Client::new();

    let question = "Explain Rust ownership in one sentence";


    let body = json!({
        "contents": [
            {
                "parts": [
                    { "text": question }
                ]
            }
        ]
    });

    let response = client.post(url).header("content-Type", "application/json").header("x-goog-api-key", API_KEY).json(&body).send().expect("Request failed");

    let response_text = response.text().expect("Failed to read response");

    println!("gemini response: \n{}", response_text);




}


