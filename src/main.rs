use std::env;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
//stock api fetch using reqwest and async using tokio

#[derive(Deserialize,Serialize)]
struct Details{ 
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}

impl Details{ 
   async fn api_fetch(args:&String,api_key:&String)->Result<Self,ExitFailure>{ 
        let url:String=format!(" https://finnhub.io/api/v1/quote?symbol={}&token={}",args,api_key);

        let _url_parse=Url::parse(&*url)?;
        let res=reqwest::get(url).await?.json::<Details>().await?;

        Ok(res)
      }
}

#[tokio::main]
async fn main()->Result<(),ExitFailure>{ 
    let api_key:String="your key ".to_string();
   let args:Vec<String>=env::args().collect();
   let mut _stock:String="".to_string();   
   if args.len()<2{ 
      println!("Not enough arguments");
   }
   else { 
     _stock=args[1].clone();
   }

   let _res= Details::api_fetch(&_stock, &api_key).await?;
   println!("{}'s current stock price: {}",_stock, _res.c);
   Ok(())

}