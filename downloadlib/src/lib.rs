use std::error::Error;
use std::io::Bytes;

pub async fn download_file(model_id: String, id: String) -> bytes::Bytes {
    let download_url = format!("https://dl.fujifilm-x.com/support/firmware/{}/{}", model_id, id);
    reqwest::get(download_url).await.unwrap().bytes().await.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_file() {
        let model_id = "x-h1-214-p07d1u27".to_string();
        let id = "FWUP0015.DAT".to_string();
        let res = download_file(model_id, id).await;
        println!("{}", res.len());
    }
    #[tokio::test]
    async fn test_download_all(){
        let model_id = "x-h1-214-p07d1u27".to_string();
        for i in 1..=15 {
            let id = format!("FWUP{:04}.DAT", i);
            let res = download_file(model_id.clone(), id).await;
            println!("{}", res.len());
            if res.len() < 5000 {
                println!("{}", String::from_utf8_lossy(&res));
            }
        }
    }
}