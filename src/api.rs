pub enum Region {
    KMS,
    MSEA,
}

pub struct MaplestoryApi {
    region: Region,
    api_key: String,
    origin: String,
}

pub struct MaplestoryApiBuilder {
    region: Option<Region>,
    api_key: Option<String>,
    origin: Option<String>,
}

impl MaplestoryApiBuilder {
    pub fn new() -> Self {
        Self {
            region: None,
            api_key: None,
            origin: None,
        }
    }

    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn origin<S: Into<String>>(mut self, origin: S) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn build(self) -> Result<MaplestoryApi, &'static str> {
        let region = self.region.unwrap_or(Region::KMS);

        let api_key = self.api_key.ok_or("api_key is required")?;

        let origin = self
            .origin
            .unwrap_or_else(|| "https://open.api.nexon.com".to_owned());

        Ok(MaplestoryApi {
            region,
            api_key,
            origin,
        })
    }
}

impl MaplestoryApi {
    pub fn builder() -> MaplestoryApiBuilder {
        MaplestoryApiBuilder::new()
    }
}
