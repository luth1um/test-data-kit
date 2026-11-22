import { describe, it, expect } from "vitest";

import { IBAN_COUNTRY, randomIban } from "../../pkg";

describe("The IBAN generator", () => {
  for (const country of Object.values(IBAN_COUNTRY).filter(
    (value): value is IBAN_COUNTRY => typeof value === "number",
  )) {
    it(`should generate an IBAN for ${IBAN_COUNTRY[country]}`, () => {
      // given
      const minLength = 15; // shortest IBAN: Norway

      // when
      const result = randomIban(country);

      // then
      expect(result.length).toBeGreaterThanOrEqual(minLength);
    });
  }
});
