use zero2prod::run;
use std::net::TcpListener;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to assign a random port");
    println!("listening on {}:{}",  listener.local_addr().unwrap().ip(), listener.local_addr().unwrap().port());
    run(listener)?.await?;
    Ok(())
}