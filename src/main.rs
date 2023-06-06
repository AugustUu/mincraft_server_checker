use tokio::{net::TcpStream, time::error::Elapsed};
use craftping::{tokio::ping, Response};
use futures::stream::FuturesUnordered;
use std::{pin::Pin, future::Future, time::{Duration, SystemTime}};
use tokio::time::timeout;
use futures::StreamExt;

async fn check(hostname:&str) -> Option<(Response,&str)>{

    if let Ok(mut stream) = TcpStream::connect((hostname, 25565)).await{

        match ping(&mut stream, hostname, 25565).await{
            Ok(result) => {
                return Some((result,hostname))
            },
            Err(_) => None,
        }
    }else{
        return None;
    }
}

#[tokio::main]
async fn main() {
    let mut servers:Vec<String> = Vec::new();
    let timer = SystemTime::now();
    for x in 0..256 {
        for y in 0..256 {
            for z in 0..8 {
                let ip = format!("45.{}.{}.{}",x,y,z);
                servers.push(ip);
            }
        }
    }
    println!("Pinging: {} ips",servers.len());

    
    let mut tasks = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<Option<(Response,&str)>,Elapsed> >>>>::new();
    let mut itr = servers.iter();

    for _ in 0..4600{
        if let Some(ip) = itr.next() {
            tasks.push(Box::pin(timeout(Duration::from_secs(1),check(ip))));
        } else {    
            break;
        }
    }

    while let Some(result) = tasks.next().await as Option<Result<Option<(Response,&str)>, Elapsed>>{

        if let Some(ip) = itr.next() {
            tasks.push(Box::pin(timeout(Duration::from_secs(1),check(ip))));
        }

        if let Ok(Some((result,ip))) = result{
            println!("IP: {} Version: {} Players: {} Discription: {}",ip,result.version,result.online_players,result.description.text);
        }

    }
    
    println!("Done {:?}",timer.elapsed().unwrap().as_secs_f64());
}
// 66
// 33
// 17
// 16

// 15 woo