mod prompt;

use prompt::{error, send_prompt, successful, successful::Role, successful::Role::*};
use std::{error::Error, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let mut prompts: Vec<(String, Role)> = Vec::new();

    loop {
        println!("\n>>>");

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt)?;
        prompt = prompt.trim().to_string();

        if prompt == "exit" {
            break;
        }

        prompts.push((prompt, User));

        println!("\nThinking...\n");
        let res = send_prompt(&prompts).await?;

        let status = res.status();
        let text = res.text().await?;
        if status != reqwest::StatusCode::OK {
            eprintln!("{:?}", text);
        }

        // println!("\n{}\n", text);

        let json: Result<successful::Response, _> = serde_json::from_str(&text);
        match json {
            Ok(json) => println!("{}", json.candidates[0].content.parts[0].text),
            Err(_) => {
                let json: error::Response = serde_json::from_str(&text)?;
                println!("Error, block reason {}", json.prompt_feedback.block_reason);
            }
        }
    }

    Ok(())
}
