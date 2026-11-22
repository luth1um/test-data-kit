import { describe, it, expect } from "vitest";

import { IBAN_COUNTRY, randomIban } from "../../pkg";

describe("The IBAN generator", () => {
  it("should have a country enum with the expected number of countries", () => {
    // given
    const countries = Object.values(IBAN_COUNTRY).filter((value): value is IBAN_COUNTRY => typeof value === "string");

    // when / then
    expect(countries).toHaveLength(3);
  });

  for (const country of Object.values(IBAN_COUNTRY).filter(
    (value): value is IBAN_COUNTRY => typeof value === "number",
  )) {
    it(`should generate an IBAN when the selected country is ${IBAN_COUNTRY[country]}`, () => {
      // given
      const minLength = 15; // shortest IBAN: Norway
      const maxLength = 33; // longest IBAN: Russia

      // when
      const result = randomIban(country);

      // then
      expect(result.length).toBeGreaterThanOrEqual(minLength);
      expect(result.length).toBeLessThanOrEqual(maxLength);
    });
  }
});
