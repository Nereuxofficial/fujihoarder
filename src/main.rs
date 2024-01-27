#[tokio::main]
fn main() {
    let res = reqwest::get("https://dl.fujifilm-x.com/support/firmware/x-h1-214-p07d1u27/FWUP0015.DAT").await.unwrap();
}
