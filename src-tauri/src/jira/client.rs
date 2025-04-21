use reqwest::{Client as HttpClient, IntoUrl, RequestBuilder};

pub struct Client {
    base_uri: String,
    user: String,
    http_client: HttpClient,
    api_key: String,
}

impl Client {
    pub fn new(base_uri: String, user: String, api_key: String) -> Self {
        let base_uri = if base_uri.ends_with("/") {
            base_uri
        } else {
            format!("{}/", &base_uri)
        };

        Self {
            base_uri,
            user,
            http_client: HttpClient::new(),
            api_key,
        }
    }

    pub fn get<U: IntoUrl>(&self, uri: U) -> RequestBuilder {
    println!("{}", self.full_uri(uri.as_str()).as_str());
        self.http_client
            .get(self.full_uri(uri))
            .basic_auth(&self.user, Some(&self.api_key))
    }

    pub fn post<U: IntoUrl>(&self, uri: U) -> RequestBuilder {
        self.http_client
            .post(self.full_uri(uri))
            .basic_auth(&self.user, Some(&self.api_key))
    }

    pub fn delete<U: IntoUrl>(&self, uri: U) -> RequestBuilder {
        self.http_client
            .delete(self.full_uri(uri))
            .basic_auth(&self.user, Some(&self.api_key))
    }

    pub fn put<U: IntoUrl>(&self, uri: U) -> RequestBuilder {
        self.http_client
            .put(self.full_uri(uri))
            .basic_auth(&self.user, Some(&self.api_key))
    }

    fn full_uri<U: IntoUrl>(&self, uri: U) -> String {
        format!("{}rest{}", &self.base_uri, uri.as_str())
    }
}