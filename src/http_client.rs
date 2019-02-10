use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::Client;

use failure::{format_err, Error};
use serde_json::Value;

const DEFAULT_UA: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.21 (KHTML, like Gecko) Mwendo/1.1.5 Safari/537.21";

pub struct UdemyHttpClient {
    access_token: String,
    client_id: String,
    client: Client,
}

pub trait HttpClient {
    fn get_as_json(&self, url: &str) -> Result<Value, Error>;
    fn get_as_data(&self, url: &str) -> Result<Vec<u8>, Error>;
    fn get_content_length(&self, url: &str) -> Result<u64, Error>;
}

impl HttpClient for UdemyHttpClient {
    fn get_as_json(&self, url: &str) -> Result<Value, Error> {
        let mut resp = self
            .client
            .get(url)
            .headers(self.construct_headers())
            .send()?;
        if resp.status().is_success() {
            Ok(resp.json()?)
        } else {
            Err(format_err!("Error while getting from url <{}>", url))
        }
    }

    fn get_content_length(&self, url: &str) -> Result<u64, Error> {
        let resp = self
            .client
            .head(url)
            .headers(self.construct_headers())
            .send()?;
        if resp.status().is_success() {
            Ok(resp
                .content_length()
                .ok_or_else(|| format_err!("Error getting length of url <{}>", url))?)
        } else {
            Err(format_err!("Error while trying to access url <{}>", url))
        }
    }

    fn get_as_data(&self, url: &str) -> Result<Vec<u8>, Error> {
        let mut resp = self
            .client
            .get(url)
            .headers(self.construct_headers())
            .send()?;
        if resp.status().is_success() {
            let mut buf: Vec<u8> = vec![];
            resp.copy_to(&mut buf)?;
            Ok(buf)
        } else {
            Err(format_err!("Error while getting from url <{}>", url))
        }
    }
}

impl UdemyHttpClient {
    pub fn new(access_token: &str, client_id: &str) -> UdemyHttpClient {
        let client = Client::new();

        UdemyHttpClient {
            client,
            access_token: String::from(access_token),
            client_id: String::from(client_id),
        }
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let bearer = format!("Bearer {}", self.access_token);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(bearer.as_str()).unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"x-udemy-authorization").unwrap(),
            HeaderValue::from_str(bearer.as_str()).unwrap(),
        );
        headers.insert(USER_AGENT, HeaderValue::from_str(DEFAULT_UA).unwrap());
        headers
    }
}
