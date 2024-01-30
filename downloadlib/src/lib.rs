use bytes::Bytes;
use log::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Firmware {
    pub os: u8,
    pub code: Bytes,
    pub version: String,
    pub lens: bool,
}

impl Firmware {
    pub fn from_raw(raw: bytes::Bytes) -> Self {
        let header = &raw[0..1024];
        let os = raw[0];
        let code_size = {
            match os {
                1 => {
                    if raw[4] == 0x12 {
                        println!("Detected S5 Pro, firmware is not supported right now");
                    }
                    64
                }
                2 => 128,
                6 => 512,
                _ => {
                    panic!("Unknown OS: {}", os);
                }
            }
        };
        // TODO: Make this nicer
        let code = Bytes::from(raw[code_size..code_size + 4].to_vec());
        let h_version =
            u32::from_le_bytes(header[code_size + 4..code_size + 8].try_into().unwrap());
        let h_version2 =
            u32::from_le_bytes(header[code_size + 8..code_size + 12].try_into().unwrap());
        let checksum =
            u32::from_le_bytes(header[code_size + 12..code_size + 16].try_into().unwrap());
        println!("Checksum: {}", hex::encode(checksum.to_le_bytes()));
        let end = u32::from_le_bytes(header[code_size + 16..code_size + 20].try_into().unwrap());
        let version: String = format!(
            "{}.{}",
            &hex::encode(h_version.to_le_bytes())[0..2].trim_start_matches('0'),
            // Only take the first 2 bytes
            &hex::encode(h_version2.to_le_bytes())[0..2]
        );
        let lens = end == 2;
        if lens {
            debug!("Detected Lens");
        }
        Self {
            os,
            code,
            version,
            lens,
        }
    }
}

pub async fn download_file(model_id: &str, id: &str) -> bytes::Bytes {
    let download_url = format!(
        "https://dl.fujifilm-x.com/support/firmware/{}/{}",
        model_id, id
    );
    reqwest::get(download_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

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
        let raw = bytes::Bytes::from(
            std::io::BufReader::new(file)
                .bytes()
                .map(|b| b.unwrap())
                .collect::<Vec<u8>>(),
        );
        let firmware = Firmware::from_raw(raw);
        assert_eq!(0x06, firmware.os);
        assert!(firmware.code.len() > 0);
        assert_eq!("2.14".to_string(), firmware.version);
        println!("{:?}", firmware);
    }
}
