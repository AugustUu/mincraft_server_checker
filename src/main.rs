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
    //servers.push("45.59.171.8".to_string() );
    //servers.push("149.56.243.144".to_string() );
    println!("Pinging: {} ips",servers.len());

    
    let mut tasks = FuturesUnordered::<Pin<Box<dyn Future<Output = Option<Response>>>>>::new();
    for server in servers.iter(){
        tasks.push(Box::pin(check(server)));
    }

    while let Some(Some(result)) = tasks.next().await as Option<Option<Response>>{
        println!("Version: {} Players: {} Discription: {}",result.version,result.online_players,result.description.text);
    }
    println!("Done");
}
