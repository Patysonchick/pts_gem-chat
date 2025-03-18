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

        match prompt.to_lowercase().as_str() {
            "/exit" => break,
            "/clear" => {
                history.clear();
                println!("\nHistory cleared!");
                continue;
            }
            _ => (),
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

        let json: Result<success::Response, _> = serde_json::from_str(&text);
        match json {
            Ok(json) => {
                let response = json.candidates[0].content.parts[0].text.clone();

                history.push(Content {
                    role: Model,
                    parts: vec![Part {
                        text: response.clone(),
                    }],
                });

                println!("{}", response);
            }
            Err(_) => {
                println!("\n{}\n", text);
                let json: error::Response = serde_json::from_str(&text)?;
                println!("Error, block reason {}", json.prompt_feedback.block_reason);
            }
        }
    }

    Ok(())
}
