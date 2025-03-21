use std::{env, env::VarError, error::Error};
use tokio::time::Duration;

pub fn env_file() -> Result<(), Box<dyn Error>> {
    let mut is_err = false;
    for i in ["API_KEY", "MODEL", "API_VERSION", "PROXY"] {
        match env::var(i) {
            Ok(_) => (),
            Err(e) => {
                match e {
                    VarError::NotPresent => eprintln!("Environment variable {} is not present", i),
                    VarError::NotUnicode(_) => {
                        eprintln!("Environment variable {} is not Unicode", i)
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

pub async fn connection() -> Result<(), Box<dyn Error>> {
    let mut client = reqwest::Client::builder().timeout(Duration::from_secs(15));

    let proxy = env::var("PROXY")?;
    if !proxy.is_empty() {
        client = client.proxy(reqwest::Proxy::all(proxy)?);
    }

    let client = client.build()?;

    match client.get("https://google.com").send().await {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(
                    format!("Connection to google.com failed, status {}", res.status()).into(),
                );
            }
        }
        Err(e) => {
            eprintln!("Connection to google.com failed:\n{}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
