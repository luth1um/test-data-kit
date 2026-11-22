use crate::iban::iban_util::iban_checksum;
use crate::util::country_info::CountryInformation;
use crate::util::random_utils::{ALL_DIGITS, ALL_DIGITS_ALL_LETTERS, push_random_chars};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = IBAN_COUNTRY)]
pub enum IbanCountry {
    GERMANY,
    ICELAND,
}

pub fn generate_random_iban(country: IbanCountry) -> Result<String, String> {
    // TODO: write tests
    match country {
        IbanCountry::GERMANY => random_iban_germany(),
        IbanCountry::ICELAND => random_iban_iceland(),
    }
}

fn random_iban_germany() -> Result<String, String> {
    // TODO: write tests
    random_simple_iban(
        CountryInformation::Germany.iso_3166_code(),
        ALL_DIGITS,
        8,
        ALL_DIGITS,
        10,
    )
}

fn random_iban_iceland() -> Result<String, String> {
    // TODO: write tests
    random_simple_iban(
        CountryInformation::Iceland.iso_3166_code(),
        ALL_DIGITS,
        7,
        ALL_DIGITS_ALL_LETTERS,
        16,
    )
}

fn random_simple_iban(
    iso_code: &str,
    bank_code_chars: &[&str],
    bank_code_len: usize,
    account_number_chars: &[&str],
    account_number_len: usize,
) -> Result<String, String> {
    // TODO: write tests
    let mut bban = String::new();
    push_random_chars(&mut bban, bank_code_chars, bank_code_len)?;
    push_random_chars(&mut bban, account_number_chars, account_number_len)?;

    let checksum = iban_checksum(iso_code, &bban)?;
    Ok(format!("{}{}{}", iso_code, checksum, bban))
}
