mod check;
mod prompt;

use clearscreen::clear;
use prompt::{Content, Part, Role::*, error, send_prompt, success};
use std::{error::Error, io, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    clear()?;

    match dotenvy::dotenv() {
        Ok(path) => println!("File {} loaded", path.display()),
        Err(e) => {
            eprintln!("File {} not found", e);
            return Err(e.into());
        }
    }
    check::env_file()?;
    check::connection().await?;

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
                clear()?;

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

        match send_prompt(history.clone()).await {
            Ok(res) => {
                let status = res.status();
                match res.text().await {
                    Ok(text) => {
                        if !status.is_success() {
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
                                let json: Result<error::Response, _> = serde_json::from_str(&text);
                                match json {
                                    Ok(json) => eprintln!(
                                        "Error, block reason {}",
                                        json.prompt_feedback.block_reason
                                    ),
                                    Err(_) => eprintln!("\n{}\n", text),
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to get text:\n{}", e),
                }
            }
            Err(e) => eprintln!("Failed send request, please try again:\n{}", e),
        }
    }

    Ok(())
}
