use hex::decode;
use std::collections::HashMap;

fn xor_buffers(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    buf1.iter().zip(buf2.iter()).map(|(&x1, &x2)| x1 ^ x2).collect()
}

fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    decode(hex_str).expect("Invalid hex string")
}

fn bytes_to_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| if b.is_ascii() { b as char } else { '.' }).collect()
}

fn main() {
    let ciphertext_hex = [
        "160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e",
        "050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740",
        "000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c",
        "0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0845124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f",
        "031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41",
        "0b4916060808001a542e0002101309050345500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c",
        "011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c",
        "0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c",
        "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    ];

    // Convert hex strings to byte vectors
    let ciphertexts: Vec<Vec<u8>> = ciphertext_hex.iter().map(|&hex| hex_to_bytes(hex)).collect();
    let min_len = ciphertexts.iter().map(|c| c.len()).min().expect("No ciphertexts provided");

    let mut key = vec![0u8; min_len];
    let mut potential_keys = vec![vec![0; 256]; min_len];

    // XOR ciphertexts in pairs and record potential keys
    for i in 0..ciphertexts.len() {
        for j in i + 1..ciphertexts.len() {
            let xored = xor_buffers(&ciphertexts[i][..min_len], &ciphertexts[j][..min_len]);
            for k in 0..xored.len() {
                if xored[k].is_ascii_alphabetic() {
                    potential_keys[k][ciphertexts[i][k] as usize ^ b' ' as usize] += 1;
                    potential_keys[k][ciphertexts[j][k] as usize ^ b' ' as usize] += 1;
                }
            }
        }
    }

    // Determine the key by selecting the most frequent potential key for each position
    for i in 0..key.len() {
        key[i] = potential_keys[i]
            .iter()
            .enumerate()
            .max_by_key(|&(_, &count)| count)
            .map(|(k, _)| k as u8)
            .unwrap_or(0);
    }

    // Decrypt the messages using the derived key
    for (i, ciphertext) in ciphertexts.iter().enumerate() {
        let decrypted_message: Vec<u8> = xor_buffers(&ciphertext[..min_len], &key);
        let decrypted_text = bytes_to_string(&decrypted_message);
        println!("Decrypted message {}: {}", i, decrypted_text);
    }

    // Print the derived key
    println!("Derived key: {}", key.iter().map(|&k| format!("{:02x}", k)).collect::<String>());


    let ciphertexts: Vec<Vec<u8>> = ciphertext_hex.iter().map(|&hex| hex_to_bytes(hex)).collect();

    // Define the known plaintext for message 7
    let known_plaintext = "Not your keys, Not your coins.";
    let known_plaintext_bytes = known_plaintext.as_bytes();

    // XOR the known plaintext with the ciphertext to obtain the key
    let mut key = Vec::new();
    let ciphertext_7 = &ciphertexts[7];
    for (i, &byte) in known_plaintext_bytes.iter().enumerate() {
        key.push(byte ^ ciphertext_7[i]);
    }

    println!("Extracted key: {}", hex::encode(&key));

    let ciphertexts: Vec<Vec<u8>> = ciphertext_hex.iter().map(|&hex| hex_to_bytes(hex)).collect();

    // Define the known plaintext for message 7
    let known_plaintext = "Not your keys, Not your coins.";
    let known_plaintext_bytes = known_plaintext.as_bytes();

    // XOR the known plaintext with the ciphertext to obtain the key part
    let mut key = vec![0xff; ciphertexts[0].len()]; // Initialize with 0xff (unknown)
    let ciphertext_7 = &ciphertexts[7];
    for (i, &byte) in known_plaintext_bytes.iter().enumerate() {
        key[i] = byte ^ ciphertext_7[i];
    }

    println!("Extracted key part: {}", hex::encode(&key));

    // Use the extracted key part to decrypt the other ciphertexts
    for (index, ciphertext) in ciphertexts.iter().enumerate() {
        let mut decrypted_message = Vec::new();
        for (i, &byte) in ciphertext.iter().enumerate() {
            let key_byte = key[i % key.len()];
            if key_byte != 0xff {
                decrypted_message.push(byte ^ key_byte);
            } else {
                decrypted_message.push(b'.'); // Placeholder for unknown parts
            }
        }
        if let Ok(decrypted_str) = std::str::from_utf8(&decrypted_message) {
            println!("Decrypted message {}: {}", index, decrypted_str);
        } else {
            println!("Decrypted message {} contains non-UTF-8 characters", index);
        }
    }

}
