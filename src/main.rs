use tokio::net::TcpStream;
use craftping::{tokio::ping, Response};
use futures::stream::FuturesUnordered;
use std::{pin::Pin, future::Future};
use futures::StreamExt;

async fn check(hostname:&str) -> Option<Response>{

    if let Ok(mut stream) = TcpStream::connect((hostname, 25565)).await{
 
        match ping(&mut stream, hostname, 25565).await{
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }else{
        return None;
    }
}

#[tokio::main]
async fn main() {
    let mut servers:Vec<String> = Vec::new();
    for x in 0..256 {
        for y in 0..8 {
            let ip = format!("45.59.{}.{}",x,y);
            servers.push(ip);
        }
    }
    println!("Pinging: {} ips",servers.len());

    
    let mut tasks = FuturesUnordered::<Pin<Box<dyn Future<Output = Option<Response>>>>>::new();
    let mut itr = servers.iter();

    for _ in 0..1000{
        if let Some(ip) = itr.next() {
            tasks.push(Box::pin(check(ip)));
        } else {    
            break;
        }
    }

    while let Some(result) = tasks.next().await as Option<Option<Response>>{

        if let Some(ip) = itr.next() {
            tasks.push(Box::pin(check(ip)));
        }

        if let Some(result)=result{
            println!("Version: {} Players: {} Discription: {}",result.version,result.online_players,result.description.text);
        }

    }
    println!("Done");
}
