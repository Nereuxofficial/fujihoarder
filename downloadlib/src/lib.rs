use std::error::Error;
use std::io::Bytes;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Firmware {
    pub os: u8,
}

impl Firmware {
    pub fn from_raw(raw: bytes::Bytes) -> Self {
        let os = raw[0];
        let code_size = {
            match os {
                1 => {
                    if raw[4] == 0x12 {
                        println!("Detected S5 Pro, firmware is not supported right now");
                        64
                    }
                }
                2 => {
                    128
                }
                6 => {
                    512
                }
                _ => {
                    panic!("Unknown OS: {}", os);
                }
            }
        };
        let code = &raw[4..=4+code_size];
        assert_eq!(code.len(), code_size);
        Self {
            os,
        }
    }
}


pub async fn download_file(model_id: &str, id: &str) -> bytes::Bytes {
    let download_url = format!("https://dl.fujifilm-x.com/support/firmware/{}/{}", model_id, id);
    reqwest::get(download_url).await.unwrap().bytes().await.unwrap()
}


#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use super::*;

    #[tokio::test]
    async fn test_download_file() {
        // x-h1-214-p07d1u27
        // ^^^^ ^^^ ^^^^^^^^
        // model number (x-h1) and firmware version (214)  and some unknown part
        let model_id = "x-h1-214-p07d1u27".to_string();
        let id = "FWUP0015.DAT".to_string();
        let res = download_file(&model_id, &id).await;
        let mut file = std::fs::File::create(model_id).unwrap();
        file.write_all(&res).unwrap();

        println!("{}", res.len());
    }


    #[test]
    fn test_firmware_from_raw() {
        let file = std::fs::File::open("x-h1-214-p07d1u27").unwrap();
        let raw = bytes::Bytes::from(std::io::BufReader::new(file).bytes().map(|b| b.unwrap()).collect::<Vec<u8>>());
        let firmware = Firmware::from_raw(raw);
        assert_eq!(firmware.os, 0x06);
    }
}