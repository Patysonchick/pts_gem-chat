mod prompt;

use prompt::{Content, Part, Role::*, error, send_prompt, success};
use std::{error::Error, io, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let mut history = Vec::new();

    loop {
        print!("\n>>> ");
        io::stdout().flush()?;

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt)?;
        prompt = prompt.trim().to_string();

        if prompt.to_lowercase() == "exit" {
            break;
        }

        history.push(Content {
            role: User,
            parts: vec![Part { text: prompt }],
        });

        println!("\nThinking...\n");
        let res = send_prompt(history.clone()).await?;

        let status = res.status();
        let text = res.text().await?;
        if status != reqwest::StatusCode::OK {
            eprintln!("{:?}", text);
        }

        // println!("\n{}\n", text);

        let json: Result<success::Response, _> = serde_json::from_str(&text);
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
