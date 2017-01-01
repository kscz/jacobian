extern crate hyper;
extern crate serde_json;

use hyper::client::Client;
use hyper::header::{Headers, Authorization, Bearer};
use std::io::Read;

#[derive(Deserialize, Debug, Default)]
pub struct VersionResponse {
    pub versions: Vec<String>
}

#[derive(Serialize, Debug, Default)]
pub struct LoginRequest {
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,

    #[serde(rename="type")]
    pub login_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct LoginResponse {
    pub access_token: String,
    pub home_server: String,
    pub user_id: String,
    pub refresh_token: Option<String>,
    pub device_id: Option<String>
}

pub struct MatrixClient {
#[macro_use]
    access_token: Option<String>,
    refresh_token: Option<String>,
    local_device_id: Option<String>,
    server_device_id: Option<String>,
    user_id: Option<String>,
    http_client: hyper::client::Client,
    homeserver: String
}

const VERSION_URL: &'static str = "/_matrix/client/versions";
const LOGIN_URL: &'static str = "/_matrix/client/r0/login";
const LOGOUT_URL: &'static str = "/_matrix/client/r0/logout";
const LOGOUT_ALL_URL: &'static str = "/_matrix/client/r0/logout/all";
const PUBLIC_ROOM_URL: &'static str = "/_matrix/client/r0/publicRooms";

impl MatrixClient {
    pub fn new(homeserver: String, device_id: Option<String>) -> MatrixClient {
        MatrixClient {
            access_token: None,
            refresh_token: None,
            local_device_id: device_id,
            server_device_id: None,
            user_id: None,
            http_client: hyper::client::Client::new(),
            homeserver: homeserver
        }
    }

    fn get_authorization_headers(&self) -> Result<Headers, String> {
        let access_token = match self.access_token {
            Some(ref x) => x.clone(),
            None => { return Err(String::from("Not logged in!")); }
        };

        let mut headers = Headers::new();
        headers.set(
                Authorization(
                    Bearer {
                        token: access_token
                    }
                )
            );

        Ok(headers)
    }

    pub fn get_supported_versions(&self) -> Result<VersionResponse, String> {
        let mut request_url = String::with_capacity(self.homeserver.len() + VERSION_URL.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(VERSION_URL);

        let mut response = match self.http_client.get(request_url.as_str()).send() {
            Ok(resp) => resp,
            Err(e) => { return Err(format!("Get request failed: {}", e)); }
        };

        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Ok(_) => (),
            Err(e) => { return Err(format!("Unable to read response body to string: {}", e)); }
        };

        match response.status {
            hyper::Ok => (),
            _ => {
                return Err(format!("Got error response from the server: {}; Contents: {}", response.status, body));
            }
        };

        let version_repsonse: VersionResponse = match serde_json::from_str(&body) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("Unable to deserialize version request. Got error {}; body: {}", e, body));
            }
        };

        Ok(version_repsonse)
    }

    pub fn login(&mut self, user: &str, password: &str) -> Result<LoginResponse, String> {
        let mut request_url = String::with_capacity(self.homeserver.len() + LOGIN_URL.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(LOGIN_URL);

        let login_request = LoginRequest {
            password: String::from(password),
            login_type: String::from("m.login.password"),
            user: Some(String::from(user)),
            medium: None,
            address: None,
            device_id: self.local_device_id.clone()
        };

        let json = match serde_json::to_string(&login_request) {
            Ok(x) => x,
            Err(e) => { return Err(format!("Couldn't serialize login request: {}", e)); }
        };

        let mut response = match self.http_client.post(request_url.as_str()).body(&json).send() {
            Ok(x) => x,
            Err(e) => { return Err(format!("Request generation failed: {}", e)); }
        };

        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Ok(_) => (),
            Err(e) => { return Err(format!("Unable to read response body to string: {}", e)); }
        };

        match response.status {
            hyper::Ok => (),
            _ => {
                return Err(format!("Got error response from the server: {}; Contents: {}", response.status, body));
            }
        };

        let login_response: LoginResponse = match serde_json::from_str(&body) {
            Ok(x) => x,
            Err(e) => { return Err(format!("Couldn't deserialize login response {}", e)); }
        };

        self.access_token = Some(login_response.access_token.clone());
        self.refresh_token = login_response.refresh_token.clone();
        self.server_device_id = login_response.device_id.clone();
        self.user_id = Some(login_response.user_id.clone());

        Ok(login_response)
    }

    pub fn logout(&mut self) -> Result<(), String> {
        let mut request_url = String::with_capacity(self.homeserver.len() + LOGOUT_URL.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(LOGOUT_URL);

        let headers = self.get_authorization_headers()?;

        let mut response = match self.http_client.post(request_url.as_str()).headers(headers).send() {
            Ok(x) => x,
            Err(e) => { return Err(format!("Request generation failed: {}", e)); }
        };

        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Ok(_) => (),
            Err(e) => { return Err(format!("Unable to read response body to string: {}", e)); }
        };

        match response.status {
            hyper::Ok => (),
            _ => {
                return Err(format!("Got error response from the server: {}; Contents: {}", response.status, body));
            }
        };

        self.access_token = None;
        self.refresh_token = None;
        self.server_device_id = None;

        Ok(())
    }

    pub fn logout_all(&mut self) -> Result<(), String> {
        let mut request_url = String::with_capacity(self.homeserver.len() + LOGOUT_ALL_URL.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(LOGOUT_ALL_URL);

        let headers = self.get_authorization_headers()?;

        let mut response = match self.http_client.post(request_url.as_str()).headers(headers).send() {
            Ok(x) => x,
            Err(e) => { return Err(format!("Request generation failed: {}", e)); }
        };

        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Ok(_) => (),
            Err(e) => { return Err(format!("Unable to read response body to string: {}", e)); }
        };

        match response.status {
            hyper::Ok => (),
            _ => {
                return Err(format!("Got error response from the server: {}; Contents: {}", response.status, body));
            }
        };

        self.access_token = None;
        self.refresh_token = None;
        self.server_device_id = None;

        Ok(())
    }
}

