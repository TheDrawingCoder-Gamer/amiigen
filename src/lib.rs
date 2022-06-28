#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use rand::Rng;
use std::io;
use std::num::ParseIntError;
// generates amiibo for use with arduino writer
pub fn gen_amiibo(amiibo_id: [u8; 8], tag_uid: &[u8]) -> io::Result<[u8; 540]> {
    if tag_uid.len() < 7 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Please use a valid 7 or 9 byte uid",
        ));
    }
    let (small_uid, bcc1, uid) = match tag_uid.len() {
        7 => {
            let bcc0 = 0x88 ^ tag_uid[0] ^ tag_uid[1] ^ tag_uid[2];
            let bcc1 = tag_uid[3] ^ tag_uid[4] ^ tag_uid[5] ^ tag_uid[6];
            Ok((
                tag_uid.try_into().unwrap(),
                bcc1,
                [
                    tag_uid[0], tag_uid[1], tag_uid[2], bcc0, tag_uid[3], tag_uid[4], tag_uid[5],
                    tag_uid[6],
                ],
            ))
        }
        9 => {
            let small_uid = [
                tag_uid[0], tag_uid[1], tag_uid[2], tag_uid[4], tag_uid[5], tag_uid[6], tag_uid[7],
            ];
            Ok((small_uid, tag_uid[8], tag_uid[0..8].try_into().unwrap())) // meant to be inclusive
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Please use a valid 7 or 9 byte uid".to_string(),
        )),
    }?;
    let pw1 = 0xAA ^ small_uid[1] ^ small_uid[3];
    let pw2 = 0x55 ^ small_uid[2] ^ small_uid[4];
    let pw3 = 0xAA ^ small_uid[3] ^ small_uid[5];
    let pw4 = 0x55 ^ small_uid[4] ^ small_uid[6];
    let mut amiibo: [u8; 540] = [0; 540];
    // set UID
    amiibo[0x1D4..0x1D4 + 8].copy_from_slice(&uid);
    amiibo[0x0..0x0 + 8].copy_from_slice(&[bcc1, 0x48, 0, 0, 0xF1, 0x10, 0xFF, 0xEE]);
    amiibo[0x28] = 0xA5; // only 1 needs to be set. others are zeroed out anyway
    amiibo[0x20B] = 0xBD;
    amiibo[0x20F] = 0x04;
    amiibo[0x210] = 0x5F;
    let mut rng = rand::thread_rng();
    rng.fill(&mut amiibo[0x1E8..0x1E8 + 32]);
    amiibo[0x1DC..0x1DC + 8].copy_from_slice(&amiibo_id);
    amiibo[0x214] = pw1;
    amiibo[0x215] = pw2;
    amiibo[0x216] = pw3;
    amiibo[0x217] = pw4;
    amiibo[0x218] = 0x80;
    amiibo[0x219] = 0x80;
    Ok(amiibo)
}
