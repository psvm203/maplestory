pub enum Region {
    KMS,
    MSEA,
}

pub struct Api {
    api_key: String,
    region: Region,
    origin: String,
}

pub struct ApiBuilder {
    api_key: Option<String>,
    region: Option<Region>,
    origin: Option<String>,
}

impl ApiBuilder {
    pub fn new() -> Self {
        Self {
            api_key: None,
            region: None,
            origin: None,
        }
    }

    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    pub fn origin<S: Into<String>>(mut self, origin: S) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn build(self) -> Result<Api, &'static str> {
        let api_key = self.api_key.ok_or("api_key is required")?;

        let region = self.region.unwrap_or(Region::KMS);

        let origin = self
            .origin
            .unwrap_or_else(|| "https://open.api.nexon.com".to_owned());

        Ok(Api {
            api_key,
            region,
            origin,
        })
    }
}

impl Api {
    pub fn builder() -> ApiBuilder {
        ApiBuilder::new()
    }
}
