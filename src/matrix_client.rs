extern crate hyper;
extern crate serde_json;

use hyper::client::Client;
use hyper::header::{Headers, Authorization, Bearer};
use std::io::{Read, Error};

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
    access_token: Option<String>,
    refresh_token: Option<String>,
    local_device_id: Option<String>,
    server_device_id: Option<String>,
    user_id: Option<String>,
    http_client: hyper::client::Client,
    homeserver: String
}

#[derive(Debug)]
pub enum MatrixClientError {
    Http(hyper::error::Error),
    Io(::std::io::Error),
    Json(serde_json::error::Error),
    Authorization(&'static str),
    BadStatus(String)
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

    fn get_authorization_headers(&self) -> Result<Headers, MatrixClientError> {
        let access_token = match self.access_token {
            Some(ref x) => x.clone(),
            None => { return Err(MatrixClientError::Authorization("Not logged in!")); }
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

    pub fn get_supported_versions(&self) -> Result<VersionResponse, MatrixClientError> {
        let mut request_url = String::with_capacity(self.homeserver.len() + VERSION_URL.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(VERSION_URL);

        let mut response = self.http_client.get(request_url.as_str()).send().map_err(MatrixClientError::Http)?;;

        let mut body = String::new();
        response.read_to_string(&mut body).map_err(MatrixClientError::Io)?;

        if hyper::Ok != response.status {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status, body)));
        }

        let version_repsonse: VersionResponse = serde_json::from_str(&body).map_err(MatrixClientError::Json)?;

        Ok(version_repsonse)
    }

    pub fn login(&mut self, user: &str, password: &str) -> Result<LoginResponse, MatrixClientError> {
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

        let json = serde_json::to_string(&login_request).map_err(MatrixClientError::Json)?;

        let mut response = self.http_client.post(request_url.as_str()).body(&json).send().map_err(MatrixClientError::Http)?;

        let mut body = String::new();
        response.read_to_string(&mut body).map_err(MatrixClientError::Io)?;

        if hyper::Ok != response.status {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status, body)));
        }

        let login_response: LoginResponse = serde_json::from_str(&body).map_err(MatrixClientError::Json)?;

        self.access_token = Some(login_response.access_token.clone());
        self.refresh_token = login_response.refresh_token.clone();
        self.server_device_id = login_response.device_id.clone();
        self.user_id = Some(login_response.user_id.clone());

        Ok(login_response)
    }

    pub fn logout(&mut self) -> Result<(), MatrixClientError> {
        let mut request_url = String::with_capacity(self.homeserver.len() + LOGOUT_URL.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(LOGOUT_URL);

        let headers = self.get_authorization_headers()?;

        let mut response = self.http_client.post(request_url.as_str()).headers(headers).send().map_err(MatrixClientError::Http)?;

        let mut body = String::new();
        response.read_to_string(&mut body).map_err(MatrixClientError::Io)?;

        if hyper::Ok != response.status {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status, body)));
        }

        self.access_token = None;
        self.refresh_token = None;
        self.server_device_id = None;

        Ok(())
    }

    pub fn logout_all(&mut self) -> Result<(), MatrixClientError> {
        let mut request_url = String::with_capacity(self.homeserver.len() + LOGOUT_ALL_URL.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(LOGOUT_ALL_URL);

        let headers = self.get_authorization_headers()?;

        let mut response = self.http_client.post(request_url.as_str()).headers(headers).send().map_err(MatrixClientError::Http)?;

        let mut body = String::new();
        response.read_to_string(&mut body).map_err(MatrixClientError::Io)?;

        if hyper::Ok != response.status {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status, body)));
        }

        self.access_token = None;
        self.refresh_token = None;
        self.server_device_id = None;

        Ok(())
    }
}

