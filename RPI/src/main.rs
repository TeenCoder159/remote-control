use std::{env, error::Error, process::exit};

mod commands;
mod stream;

use stream::{Stream, start_server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ip_addr = get_env("SERVER");
    let listener = start_server(&ip_addr).await;
    for stream_err in listener.incoming() {
        let mut stream = stream_err?;

        stream
            .write_to_stream("Hello from your Server. Connection was established succesfully")
            .await?;

        let _command = stream.read_stream().await?;
    }
    Ok(())
}

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(a) => a,
        Err(e) => {
            eprintln!(
                "Unable to access {key} environment variable: {e}\nPlease try setting your {key} environment variable"
            );
            exit(-1);
        }
    }
}
