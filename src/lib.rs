mod iban;
mod util;

pub use iban::iban_gen::IbanCountry;

use crate::iban::iban_gen::generate_random_iban;
use wasm_bindgen::prelude::wasm_bindgen;

/// Generates a random IBAN satisfying checksum constraints for the specified country.
///
/// @param country the country for which the IBAN will be generated
/// @returns a random IBAN for the specified country
/// @throws {Error} if the generator encounters an unexpected internal issue
///
/// @example
/// const iban = randomIban(IBAN_COUNTRY.GERMANY);
/// console.log(iban); // e.g., 'DE41576764994824777093'
#[wasm_bindgen(js_name = randomIban)]
pub fn random_iban(country: IbanCountry) -> String {
    match generate_random_iban(country) {
        Ok(iban) => iban,
        Err(msg) => wasm_bindgen::throw_str(&msg),
    }
}
