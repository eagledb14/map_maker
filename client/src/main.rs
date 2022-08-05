use std::thread::sleep;
use std::time::Duration;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let res: Vec<f32> = reqwest::get("http://localhost:8080").await?.json().await?;
    //let map: Vec<f32> = serde_json::from_str(&res).unwrap();
    println!("{:?}", res); 
    sleep(Duration::from_millis(500));
    Ok(())
}
