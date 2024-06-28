use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};
use std::env;
use dotenv::dotenv;

#[derive(Serialize)]
struct RequestBody {
    inputs: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    generated_text: String,
}

async fn get_llm_response(query: String) -> Result<(), Box<dyn Error>> {
    // Set up the HTTP client
    let client = Client::new();

    // Define the URL and the API token
    let url = "https://api-inference.huggingface.co/models/mistralai/Mistral-7B-Instruct-v0.3";
    let api_token = env::var("HUGGINGFACE_API_TOKEN").expect("API_TOKEN must be set");

    // Create the request body
    let request_body = RequestBody {
        inputs: query,
    };

    // Send the POST request
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&request_body)
        .send()
        .await?
        .json::<Vec<ResponseBody>>()
        .await?;

    // Print the response
    if let Some(resp) = response.get(0) {
        println!("\nModel Response: {}\n", resp.generated_text);
    } else {
        println!("No response received from the model");
    }

    Ok(())
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    // welcome message
    println!("Hi, I'm a Rust chatbot! How can I help you today?");

    loop {
        print!("--------------------------------------\n");
        print!("User: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut query = String::new();
        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");

            if query.chars().all(char::is_whitespace) {
            println!("Empty query, please enter a valid query.");
            continue;
        }

        // println!("You said: {}", query.trim());
        if let Err(e)  = get_llm_response(query).await {
            eprintln!("Error: {}", e);
        }
    }
}
