use tokio::{net::TcpStream, time::error::Elapsed};
use craftping::{tokio::ping, Response};
use futures::stream::FuturesUnordered;
use std::{pin::Pin, future::Future, time::{Duration, SystemTime}};
use tokio::time::timeout;
use futures::StreamExt;

async fn check(hostname:&str) -> Option<(Response,&str)>{

    if let Ok(Ok(mut stream)) = timeout(Duration::from_secs(1), TcpStream::connect((hostname, 25565))).await{

        match timeout(Duration::from_secs(1), ping(&mut stream, hostname, 25565)).await{
            Ok(Ok(result)) => Some((result,hostname)),
            Ok(Err(_)) => None,
            Err(_) => None,
        }
    }else{
        return None;
    }
}

const PLAYER_FINDER: bool = false;

#[tokio::main]
async fn main() {
    let mut servers:Vec<String> = Vec::new();
    let timer = SystemTime::now();
    for x in 0..256 {
        for y in 0..256 {
            //for z in 57..60 {
                let ip = format!("45.59.{}.{}",x,y);
                servers.push(ip);
            //}
        }
    }
    println!("Pinging: {} ips",servers.len());

    
    let mut tasks = FuturesUnordered::<Pin<Box<dyn Future<Output = Option<(Response,&str)>>>>>::new();
    let mut itr = servers.iter();

    for _ in 0..40000{
        if let Some(ip) = itr.next() {
            tasks.push(Box::pin(check(ip)));
        } else {    
            break;
        }
    }

    while let Some(result) = tasks.next().await as Option<Option<(Response,&str)>>{

        if let Some(ip) = itr.next() {
            tasks.push(Box::pin(check(ip)));
        }

        if let Some((result,ip)) = result as Option<(Response,&str)>{
            if PLAYER_FINDER{
                if let Some(players) = result.sample{
                    for player in players.iter(){
                        println!("{}            {}",player.name,result.description.text)
                    }
                }
            }else{
                println!("IP: {} Version: {} Players: {} Discription: {}",ip,result.version,result.online_players,result.description.text);
            }
        }

    }
    
    println!("Done {:?}",timer.elapsed().unwrap().as_secs_f64());
}
// 66
// 33
// 17
// 16
// 10


// 15 woo (releasse)
// 9.5