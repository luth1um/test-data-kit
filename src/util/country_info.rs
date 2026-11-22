pub enum CountryInformation {
    Germany,
    Iceland,
}

impl CountryInformation {
    pub fn iso_3166_code(&self) -> &'static str {
        // TODO: write tests
        match self {
            CountryInformation::Germany => "DE",
            CountryInformation::Iceland => "IS",
        }
    }
}
