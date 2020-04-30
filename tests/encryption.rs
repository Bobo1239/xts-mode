use std::fs;

#[macro_use]
extern crate hex_literal;

use aes::Aes128;

use block_cipher_trait::BlockCipher;
use xts_mode::{get_tweak_default, Xts128};

#[test]
fn recrypt() {
    let key = hex!("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f");
    let plaintext = b"Yu9b5QgBck wBogw5ATwAHLEV YWDPK2mS";
    assert_eq!(plaintext.len(), 34);
    let mut buffer = plaintext.to_owned();

    let cipher_1 = Aes128::new_varkey(&key[..16]).unwrap();
    let cipher_2 = Aes128::new_varkey(&key[16..]).unwrap();

    let xts = Xts128::<Aes128>::new(cipher_1, cipher_2);

    let tweak = get_tweak_default(0);
    xts.encrypt_sector(&mut buffer, tweak);
    let _encrypted = buffer.clone();
    xts.decrypt_sector(&mut buffer, tweak);

    assert_eq!(&buffer[..], &plaintext[..]);
}

#[test]
fn recrypt_no_remainder() {
    let key = hex!("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f");
    let plaintext = b"ATwAHLEVk WDPK2m5D1ZY9QpLyW 3aK9";
    assert_eq!(plaintext.len(), 32);
    let mut buffer = plaintext.to_owned();

    let cipher_1 = Aes128::new_varkey(&key[..16]).unwrap();
    let cipher_2 = Aes128::new_varkey(&key[16..]).unwrap();

    let xts = Xts128::<Aes128>::new(cipher_1, cipher_2);

    let tweak = get_tweak_default(0);
    xts.encrypt_sector(&mut buffer, tweak);
    let _encrypted = buffer.clone();
    xts.decrypt_sector(&mut buffer, tweak);

    assert_eq!(&buffer[..], &plaintext[..]);
}

// Seems like OpenSSL resets the tweak every 0x1000 bytes
fn get_tweak_openssl(_sector_index: u128) -> [u8; 0x10] {
    [0; 0x10]
}

#[test]
fn encrypt_file_no_remainder() {
    let key = hex!("f1e4acd1ca1258b2751c538f512cd8d2d26d7867a3e1245c5c4462cd398d443e");
    let mut buffer = fs::read("test_files/random_no_remainder").expect("could not read input");
    assert_eq!(buffer.len(), 0x3000);

    let cipher_1 = Aes128::new_varkey(&key[..16]).unwrap();
    let cipher_2 = Aes128::new_varkey(&key[16..]).unwrap();

    let xts = Xts128::<Aes128>::new(cipher_1, cipher_2);

    xts.encrypt_area(&mut buffer, 0x1000, 0, get_tweak_openssl);

    let reference =
        fs::read("test_files/random_no_remainder.enc").expect("could not read reference");
    assert_eq!(&buffer[..], &reference[..]);
}

#[test]
fn decrypt_file_no_remainder() {
    let key = hex!("f1e4acd1ca1258b2751c538f512cd8d2d26d7867a3e1245c5c4462cd398d443e");
    let mut buffer = fs::read("test_files/random_no_remainder.enc").expect("could not read input");
    assert_eq!(buffer.len(), 0x3000);

    let cipher_1 = Aes128::new_varkey(&key[..16]).unwrap();
    let cipher_2 = Aes128::new_varkey(&key[16..]).unwrap();

    let xts = Xts128::<Aes128>::new(cipher_1, cipher_2);

    xts.decrypt_area(&mut buffer, 0x1000, 0, get_tweak_openssl);

    let reference = fs::read("test_files/random_no_remainder").expect("could not read reference");
    assert_eq!(&buffer[..], &reference[..]);
}

#[test]
fn encrypt_file_with_remainder() {
    let key = hex!("a5f85b18e5d06f13aa3a2dca389d776ab195a6feb1827980eb00abb0f75ea609");
    let mut buffer = fs::read("test_files/random_with_remainder").expect("could not read input");
    assert_eq!(buffer.len(), 20001);

    let cipher_1 = Aes128::new_varkey(&key[..16]).unwrap();
    let cipher_2 = Aes128::new_varkey(&key[16..]).unwrap();

    let xts = Xts128::<Aes128>::new(cipher_1, cipher_2);

    xts.encrypt_area(&mut buffer, 0x1000, 0, get_tweak_openssl);

    let reference =
        fs::read("test_files/random_with_remainder.enc").expect("could not read reference");
    assert_eq!(&buffer[..], &reference[..]);
}

#[test]
fn decrypt_file_with_remainder() {
    let key = hex!("a5f85b18e5d06f13aa3a2dca389d776ab195a6feb1827980eb00abb0f75ea609");
    let mut buffer =
        fs::read("test_files/random_with_remainder.enc").expect("could not read input");
    assert_eq!(buffer.len(), 20001);

    let cipher_1 = Aes128::new_varkey(&key[..16]).unwrap();
    let cipher_2 = Aes128::new_varkey(&key[16..]).unwrap();

    let xts = Xts128::<Aes128>::new(cipher_1, cipher_2);

    xts.decrypt_area(&mut buffer, 0x1000, 0, get_tweak_openssl);

    let reference = fs::read("test_files/random_with_remainder").expect("could not read reference");
    assert_eq!(&buffer[..], &reference[..]);
}
