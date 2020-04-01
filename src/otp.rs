pub struct OTP;

impl OTP {
    fn xor_string(msg: &String, key: &String, cipher: &mut String) {
        let mut msg_iter = msg.as_bytes().iter();
        let mut key_iter = key.as_bytes().iter();
        let mut msg_chunk = msg_iter.next();
        let mut key_chunk = key_iter.next();

        while msg_chunk.is_some() && key_chunk.is_some() {
            let cipher_chunk = (msg_chunk.unwrap() ^ key_chunk.unwrap()) as char;
            cipher.push(cipher_chunk); 
            msg_chunk = msg_iter.next();
            key_chunk = key_iter.next();
        }
    }

    pub fn encode(msg: &String, key: &String) -> String {
        let mut cipher: String = String::new();
        {
            Self::xor_string(msg, key, &mut cipher);
        }
        cipher
    }

    pub fn decode(cipher: &String, key: &String) -> String {
        let mut msg: String = String::new();
        {
            Self::xor_string(cipher, key, &mut msg);
        }
        msg
    }
}
