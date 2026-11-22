mod iban;
mod util;

pub use iban::iban_gen::IbanCountry;

use crate::iban::iban_gen::generate_random_iban;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = randomIban)]
pub fn random_iban(country: IbanCountry) -> String {
    match generate_random_iban(country) {
        Ok(iban) => iban,
        Err(msg) => wasm_bindgen::throw_str(&msg),
    }
}
