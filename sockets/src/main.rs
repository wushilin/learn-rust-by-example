use std::env::args;
use std::error::Error;
use std::fs::remove_file;
use std::io;
use std::time::Duration;

use async_std::task;
use async_std::net::TcpListener;
use async_std::prelude::*;

use async_listen::{ListenExt, ByteStream, backpressure, error_hint};

mod buffer;

use crate::buffer::{Buffer, Factory};

fn main() -> Result<(), Box<dyn Error>> {
    let (_, bp) = backpressure::new(10);
    let BP:buffer::ByteArrayFactory = buffer::new_byte_array_factory(1024, 100);

    task::block_on(async {
        let listener = TcpListener::bind("localhost:8080").await?;
        eprintln!("Accepting connections on localhost:8080");
        let mut incoming = listener.incoming()
            .log_warnings(log_accept_error)
            .handle_errors(Duration::from_millis(500))
            .backpressure_wrapper(bp);
        

        while let Some(stream) = incoming.next().await {
            task::spawn(async {
                if let Err(e) = connection_loop(stream).await {
                    eprintln!("Error: {}", e);
                }
            });
        }
        Ok(())
    })
}

async fn connection_loop(mut stream: ByteStream) -> Result<(), io::Error> {
    println!("Connected from {}", stream.peer_addr()?);
    //task::sleep(Duration::from_secs(5)).await;
    //let buf = bp.Create();
    let mut buf = [0 as u8; 10];
    loop {
        let rr = stream.read(&mut buf).await?;
        println!("{} bytes read!", rr);
        if rr == 0 {
            break;
        }
        stream.write_all(&buf[..rr]).await?;
        println!("{} bytes written!", rr);
    }
    Ok(())
}

fn log_accept_error(e: &io::Error) {
    eprintln!("Accept error: {}. Sleeping 0.5s. {}", e, error_hint(&e));
}