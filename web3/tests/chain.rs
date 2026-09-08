use premise_web3::{address, address_from_text, address_parts, address_text, checksummed, hashed, short_label, Address, ZERO_ADDRESS};
use patterns::{hex_text, recorded, restored, Field};

const TRANSFER_TOPIC: &str = "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
const CHECKSUMMED: &str = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";

#[test]
fn the_keccak_of_a_signature_is_its_topic() {
    assert_eq!(hex_text(&hashed(b"Transfer(address,address,uint256)")), TRANSFER_TOPIC);
}

#[test]
fn an_address_keeps_its_bytes_and_shows_its_checksum() {
    let parsed = address_from_text(CHECKSUMMED).expect("a valid address");
    assert_eq!(address_text(&parsed), CHECKSUMMED.to_lowercase());
    assert_eq!(checksummed(&parsed), CHECKSUMMED);
    assert_eq!(address_from_text("0x1234"), None);
    assert_eq!(address_from_text(&CHECKSUMMED.to_uppercase().replace("0X", "0x")), Some(parsed));
    assert_eq!(short_label(&parsed, 6), "0x5aae...");
}

#[test]
fn an_address_is_two_numbers_to_a_spec() {
    let built = address(0x00112233445566778899aabbccddeeff, 0x01020304);
    assert_eq!(address_parts(&built), (0x00112233445566778899aabbccddeeff, 0x01020304));
    assert_eq!(address_text(&built), "0x00112233445566778899aabbccddeeff01020304");
    assert_eq!(address_parts(&ZERO_ADDRESS), (0, 0));
}

#[test]
fn an_address_records_as_its_lowercase_text() {
    let parsed: Address = address_from_text(CHECKSUMMED).expect("a valid address");
    assert_eq!(recorded(&parsed), Field::Text(CHECKSUMMED.to_lowercase()));
    assert_eq!(restored::<Address>(&Field::Text(String::from(CHECKSUMMED))), Some(parsed));
    assert_eq!(restored::<Address>(&Field::Text(String::from("0x12"))), None);
}
