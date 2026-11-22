use crate::iban::iban_util::iban_checksum;
use crate::util::country_info::CountryInformation;
use crate::util::random_utils::{ALL_DIGITS, push_random_chars};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = IBAN_COUNTRY)]
pub enum IbanCountry {
    GERMANY,
}

pub fn generate_random_iban(country: IbanCountry) -> Result<String, String> {
    // TODO: write tests
    match country {
        IbanCountry::GERMANY => random_german_iban(),
    }
}

fn random_german_iban() -> Result<String, String> {
    // TODO: write tests
    random_simple_iban(
        CountryInformation::Germany.iso_3166_code(),
        ALL_DIGITS,
        8,
        ALL_DIGITS,
        10,
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
