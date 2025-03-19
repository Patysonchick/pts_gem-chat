mod prompt;

use prompt::{Content, Part, Role::*, error, send_prompt, success};
use std::{env, env::VarError, error::Error, io, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match dotenvy::dotenv() {
        Ok(path) => println!("File {} loaded", path.display()),
        Err(e) => {
            println!("File {} not found", e);
            return Err(e.into());
        }
    }
    check_env_file()?;

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

        match send_prompt(history.clone()).await {
            Ok(res) => {
                let status = res.status();
                match res.text().await {
                    Ok(text) => {
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
                                let json: Result<error::Response, _> = serde_json::from_str(&text);
                                match json {
                                    Ok(json) => println!(
                                        "Error, block reason {}",
                                        json.prompt_feedback.block_reason
                                    ),
                                    Err(_) => println!("\n{}\n", text),
                                }
                            }
                        }
                    }
                    Err(e) => println!("Failed to get text: {}", e),
                }
            }
            Err(e) => println!("Failed send request, please try again:\n{}", e),
        }
    }

    Ok(())
}

fn check_env_file() -> Result<(), Box<dyn Error>> {
    let mut is_err = false;
    for i in ["API_KEY", "MODEL", "API_VERSION", "PROXY"] {
        match env::var(i) {
            Ok(_) => (),
            Err(e) => {
                match e {
                    VarError::NotPresent => println!("Environment variable {} is not present", i),
                    VarError::NotUnicode(_) => {
                        println!("Environment variable {} is not Unicode", i)
                    }
                }
                is_err = true;
            }
        }
    }

    if is_err {
        return Err("Environment variable not set".into());
    }

    Ok(())
}
