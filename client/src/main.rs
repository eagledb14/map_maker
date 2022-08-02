use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    loop{
        let res = reqwest::get("http://localhost:8080").await?;
        println!("{:?}", res.text().await);
        let res = reqwest::get("http://localhost:8080/inc").await?;
        println!("{:?}", res.text().await);
        sleep(Duration::from_millis(500));
    }
    Ok(())
}
