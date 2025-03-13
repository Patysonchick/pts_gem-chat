mod prompt;

use prompt::{Response, send_prompt};
use std::{error::Error, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    loop {
        println!(">>> ");

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt)?;

        if prompt.trim() == "exit" {
            break;
        }

        println!("\nThinking...\n");
        let res = send_prompt(&prompt).await?;

        let status = res.status();
        let text = res.text().await?;
        if status != reqwest::StatusCode::OK {
            eprintln!("{:?}", text);
        }

        let json: Response = serde_json::from_str(&text)?;

        println!("{}", json.candidates[0].content.parts[0].text);
    }

    Ok(())
}
