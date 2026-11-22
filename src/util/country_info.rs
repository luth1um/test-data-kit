pub enum CountryInformation {
    Germany,
    Iceland,
    Latvia,
}

impl CountryInformation {
    pub fn iso_3166_1_alpha_2(&self) -> &'static str {
        // TODO: write tests
        match self {
            CountryInformation::Germany => "DE",
            CountryInformation::Iceland => "IS",
            CountryInformation::Latvia => "LV",
        }
    }
}
