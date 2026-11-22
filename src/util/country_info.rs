pub enum CountryInformation {
    Germany,
}

impl CountryInformation {
    pub fn iso_3166_code(&self) -> &'static str {
        // TODO: write tests
        match self {
            CountryInformation::Germany => "DE",
        }
    }
}
