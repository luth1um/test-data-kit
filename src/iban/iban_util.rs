pub fn iban_checksum(iso_code: &str, bban: &str) -> Result<String, String> {
    // TODO: write tests
    if iso_code.len() != 2 || !iso_code.is_ascii() {
        return Err("Invalid country code".to_owned());
    }

    if !iso_code.bytes().all(|b| b.is_ascii_alphabetic()) {
        return Err("Country code must contain two letters".to_owned());
    }

    let mut remainder: u32 = 0;

    // 1. Move country code and checksum placeholder "00" to the end
    // 2. Replace letters with numbers (A=10, ..., Z=35)
    // 3. Calculate 98 - (number mod 97)
    for c in bban
        .bytes()
        .chain(iso_code.bytes())
        .chain(b"00".iter().copied())
    {
        let value = match c {
            b'0'..=b'9' => (c - b'0') as u32,
            b'A'..=b'Z' => (c - b'A' + 10) as u32,
            _ => return Err("Invalid character in BBAN".to_owned()),
        };

        if value < 10 {
            remainder = (remainder * 10 + value) % 97;
        } else {
            remainder = (remainder * 10 + value / 10) % 97;
            remainder = (remainder * 10 + value % 10) % 97;
        }
    }

    let checksum = 98 - remainder;

    Ok(format!("{:02}", checksum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iban_checksum_returns_correct_checksum_when_input_is_valid() {
        // given
        let cases = [
            ("DE", "924696032812692945", "93"),
            ("ES", "07378757017304329924", "02"),
            ("IT", "G0847750497HYAIY4JB9PVO", "19"),
            ("NL", "PVQI1398326933", "49"),
            ("VA", "241867884564447636", "08"),
        ];

        for (iso_code, bban, expect) in cases {
            // when
            let result = iban_checksum(iso_code, bban);

            // then
            assert_eq!(result.unwrap(), expect);
        }
    }
}
