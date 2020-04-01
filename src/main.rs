mod otp;

use otp::OTP;

fn main() {
    let key = "key".to_string();
    let msg = "msg".to_string();
    let enc = OTP::encode(&msg, &key);
    let dec = OTP::decode(&enc, &key);

    assert_eq!(msg, dec);
}
