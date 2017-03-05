extern crate serde_json;
extern crate reqwest;

use std::collections::HashMap;

use reqwest::{Client, UrlError};
use reqwest::header::{Headers, Authorization, Bearer};
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

#[derive(Deserialize, Debug, Default)]
pub struct PublicRoomsChunk {
    pub world_readable: bool,
    pub topic: Option<String>,
    pub num_joined_members: i64,
    pub avatar_url: String,
    pub room_id: String,
    pub guest_can_join: bool,
    pub aliases: Option<Vec<String>>,
    pub name: String
}

#[derive(Deserialize, Debug, Default)]
pub struct PublicRoomsResponse {
    pub start: Option<String>,
    pub chunk: Vec<PublicRoomsChunk>,
    pub end: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct JoinResponse {
    pub room_id: String
}

#[derive(Deserialize, Debug, Default)]
pub struct UnreadNotificationCounts {
    pub highlight_count: Option<i64>,
    pub notification_count: Option<i64>
}

#[derive(Deserialize, Debug, Default)]
pub struct Unsigned<T> {
    pub prev_content: Option<T>,
    pub age: i64,
    pub transaction_id: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Event {
    Unknown,

    #[serde(rename = "m.typing")]
    Typing(TypingEvent),

    #[serde(rename = "m.room.power_levels")]
    RoomPowerLevels(RoomPowerLevelsEvent),

    #[serde(rename = "m.room.history_visibility")]
    RoomHistoryVisibility(RoomHistoryVisibilityEvent),

    #[serde(rename = "m.room.topic")]
    RoomTopic(RoomTopicEvent),

    #[serde(rename = "m.receipt")]
    Receipt(ReceiptEvent),

    #[serde(rename = "m.presence")]
    Presence(PresenceEvent),

    #[serde(rename = "m.room.member")]
    RoomMember(RoomMemberEvent),

    #[serde(rename = "m.room.aliases")]
    RoomAlias(RoomAliasEvent),

    #[serde(rename = "m.room.canonical_alias")]
    RoomCanonicalAlias(RoomCanonicalAliasEvent),

    #[serde(rename = "m.room.create")]
    RoomCreate(RoomCreateEvent),

    #[serde(rename = "m.room.avatar")]
    RoomAvatar(RoomAvatarEvent),

    #[serde(rename = "m.room.join_rules")]
    RoomJoinRules(RoomJoinRulesEvent),

    #[serde(rename = "m.room.message")]
    RoomMessage(RoomMessageEvent),

    #[serde(rename = "m.room.name")]
    RoomName(RoomNameEvent),
}

impl Default for Event {
    fn default() -> Event {
        Event::Unknown
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ReceiptEvent {
}

#[derive(Deserialize, Debug)]
#[serde(tag = "msgtype")]
pub enum RoomMessageTypes {
    Unknown,

    #[serde(rename = "m.audio")]
    AudioMessage(AudioMessageType),

    #[serde(rename = "m.video")]
    VideoMessage(VideoMessageType),

    #[serde(rename = "m.location")]
    LocationMessage(LocationMessageType),

    #[serde(rename = "m.file")]
    FileMessage(FileMessageType),

    #[serde(rename = "m.image")]
    ImageMessage(ImageMessageType),

    #[serde(rename = "m.notice")]
    NoticeMessage(NoticeMessageType),

    #[serde(rename = "m.emote")]
    EmoteMessage(EmoteMessageType),

    #[serde(rename = "m.text")]
    TextMessage(TextMessageType)
}

impl Default for RoomMessageTypes {
    fn default() -> RoomMessageTypes {
        RoomMessageTypes::Unknown
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomMessageEvent {
    pub content: RoomMessageTypes,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomMessageTypes>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct AudioInfo {
    pub mimetype: Option<String>,
    pub duration: Option<i64>,
    pub size: Option<i64>
}

#[derive(Deserialize, Debug, Default)]
pub struct AudioMessageType {
    pub body: String,
    pub info: Option<AudioInfo>,
    pub url: String
}

#[derive(Deserialize, Debug, Default)]
pub struct VideoInfo {
    pub mimetype: Option<String>,
    pub thumbnail_info: Option<ImageInfo>,
    pub h: Option<i64>,
    pub thumbnail_url: Option<String>,
    pub w: Option<i64>,
    pub duration: Option<i64>,
    pub size: Option<i64>
}

#[derive(Deserialize, Debug, Default)]
pub struct VideoMessageType {
    pub body: String,
    pub info: Option<VideoInfo>,
    pub url: String
}

#[derive(Deserialize, Debug, Default)]
pub struct LocationMessageType {
    pub body: String,
    pub thumbnail_info: Option<ImageInfo>,
    pub geo_uri: String,
    pub thumbnail_url: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct FileInfo {
    pub mimetype: Option<String>,
    pub size: Option<i64>
}

#[derive(Deserialize, Debug, Default)]
pub struct FileMessageType {
    pub body: String,
    pub info: Option<FileInfo>,
    pub thumbnail_info: Option<ImageInfo>,
    pub url: String,
    pub filename: String,
    pub thumbnail_url: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct TextMessageType {
    pub body: String
}

#[derive(Deserialize, Debug, Default)]
pub struct EmoteMessageType {
    pub body: String
}

#[derive(Deserialize, Debug, Default)]
pub struct NoticeMessageType {
    pub body: String
}

#[derive(Deserialize, Debug, Default)]
pub struct ImageMessageType {
    pub body: String,
    pub info: Option<ImageInfo>,
    pub thumbnail_info: Option<ImageInfo>,
    pub url: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomJoinRulesEventContent {
    pub join_rule: String
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomJoinRulesEvent {
    pub content: RoomJoinRulesEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomJoinRulesEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomCreateEventContent {
    pub creator: String,
    #[serde(rename = "m.federate")]
    pub federate: Option<bool>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomCreateEvent {
    pub content: RoomCreateEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomCreateEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct ImageInfo {
    pub mimetype: Option<String>,
    pub h: Option<i64>,
    pub w: Option<i64>,
    pub size: Option<i64>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomAvatarEventContent {
    pub url: String,
    pub info: Option<ImageInfo>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_info: Option<ImageInfo>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomAvatarEvent {
    pub content: RoomAvatarEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomAvatarEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomAliasEventContent {
    pub aliases: Vec<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomAliasEvent {
    pub content: RoomAliasEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomAliasEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomCanonicalAliasEventContent {
    pub alias: String
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomCanonicalAliasEvent {
    pub content: RoomCanonicalAliasEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomCanonicalAliasEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomNameEventContent {
    pub name: String
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomNameEvent {
    pub content: RoomNameEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomNameEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomTopicEventContent {
    pub topic: String
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomTopicEvent {
    pub content: RoomTopicEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomTopicEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct TypingEventContent {
    pub user_ids: Vec<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct TypingEvent {
    pub content: TypingEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<TypingEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomPowerLevelsEventContent {
    pub events_default: Option<i64>,
    pub invite: Option<i64>,
    pub state_default: Option<i64>,
    pub redact: Option<i64>,
    pub ban: Option<i64>,
    pub users_default: Option<i64>,
    pub events: HashMap<String, i64>,
    pub kick: Option<i64>,
    pub users: HashMap<String, i64>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomPowerLevelsEvent {
    pub content: RoomPowerLevelsEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomPowerLevelsEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomHistoryVisibilityEventContent {
    pub history_visibility: String
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomHistoryVisibilityEvent {
    pub content: RoomHistoryVisibilityEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomHistoryVisibilityEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomMemberEventContent {
    pub membership: String,
    pub avatar_url: Option<String>,
    pub displayname: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomMemberEvent {
    pub content: RoomMemberEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<RoomMemberEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct PresenceEventContent {
    pub user_id: Option<String>,
    pub presence: String,
    pub avatar_url: Option<String>,
    pub last_active_ago: Option<i64>,
    pub currently_active: Option<bool>,
    pub displayname: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct PresenceEvent {
    pub content: PresenceEventContent,
    pub origin_server_ts: Option<i64>,
    pub sender: Option<String>,
    pub unsigned: Option<Unsigned<PresenceEventContent>>,
    pub state_key: Option<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct State {
    pub events: Vec<Event>
}

#[derive(Deserialize, Debug, Default)]
pub struct Presence {
    pub events: Vec<Event>
}

#[derive(Deserialize, Debug, Default)]
pub struct InviteState {
    pub events: Vec<Event>
}

#[derive(Deserialize, Debug, Default)]
pub struct AccountData {
    pub events: Vec<Event>
}

#[derive(Deserialize, Debug, Default)]
pub struct Ephemeral {
    pub events: Vec<Event>
}

#[derive(Deserialize, Debug, Default)]
pub struct InvitedState {
    pub events: Vec<Event>
}

#[derive(Deserialize, Debug, Default)]
pub struct Timeline {
    pub limited: bool,
    pub prev_batch: Option<String>,
    pub events: Vec<Event>
}

#[derive(Deserialize, Debug, Default)]
pub struct JoinedRoom {
    pub unread_notifications: Option<UnreadNotificationCounts>,
    pub timeline: Option<Timeline>,
    pub state: Option<State>,
    pub account_data: Option<AccountData>,
    pub ephemeral: Option<Ephemeral>
}

#[derive(Deserialize, Debug, Default)]
pub struct LeftRoom {
    pub timeline: Option<Timeline>,
    pub state: Option<State>
}

#[derive(Deserialize, Debug, Default)]
pub struct InvitedRoom {
    pub invite_state: Option<InviteState>
}

#[derive(Deserialize, Debug, Default)]
pub struct Rooms {
    pub leave: HashMap<String, LeftRoom>,
    pub join: HashMap<String, JoinedRoom>,
    pub invite: HashMap<String, InvitedRoom>
}

#[derive(Deserialize, Debug, Default)]
pub struct SyncResponse {
    pub next_batch: Option<String>,
    pub rooms: Rooms,
    pub presence: Presence
}

pub struct MatrixClient {
    access_token: Option<String>,
    refresh_token: Option<String>,
    local_device_id: Option<String>,
    server_device_id: Option<String>,
    user_id: Option<String>,
    http_client: reqwest::Client,
    homeserver: String
}

#[derive(Debug)]
pub enum MatrixClientError {
    UrlError(reqwest::UrlError),
    Http(reqwest::Error),
    Io(::std::io::Error),
    Json(serde_json::error::Error),
    NotLoggedIn,
    BadStatus(String),
    BadRoomId(String)
}

const VERSION_URL: &'static str = "/_matrix/client/versions";
const LOGIN_URL: &'static str = "/_matrix/client/r0/login";
const LOGOUT_URL: &'static str = "/_matrix/client/r0/logout";
const LOGOUT_ALL_URL: &'static str = "/_matrix/client/r0/logout/all";
const PUBLIC_ROOM_URL: &'static str = "/_matrix/client/r0/publicRooms";
const JOIN_ROOM_URL: &'static str = "/_matrix/client/r0/join/";
const SYNC_URL: &'static str = "/_matrix/client/r0/sync";

impl MatrixClient {
    pub fn new(homeserver: String, device_id: Option<String>) -> MatrixClient {
        MatrixClient {
            access_token: None,
            refresh_token: None,
            local_device_id: device_id,
            server_device_id: None,
            user_id: None,
            http_client: reqwest::Client::new().unwrap(),
            homeserver: homeserver
        }
    }

    fn get_authorization_headers(&self) -> Result<Headers, MatrixClientError> {
        let access_token = match self.access_token {
            Some(ref x) => x.clone(),
            None => { return Err(MatrixClientError::NotLoggedIn); }
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

        if reqwest::StatusCode::Ok != *response.status() {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status(), body)));
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

        let mut response = self.http_client.post(request_url.as_str()).body(json).send().map_err(MatrixClientError::Http)?;

        let mut body = String::new();
        response.read_to_string(&mut body).map_err(MatrixClientError::Io)?;

        if reqwest::StatusCode::Ok != *response.status() {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status(), body)));
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

        if reqwest::StatusCode::Ok != *response.status() {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status(), body)));
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

        if reqwest::StatusCode::Ok != *response.status() {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status(), body)));
        }

        self.access_token = None;
        self.refresh_token = None;
        self.server_device_id = None;

        Ok(())
    }

    pub fn list_public_rooms(&self) -> Result<PublicRoomsResponse, MatrixClientError> {
        let mut request_url = String::with_capacity(self.homeserver.len() + PUBLIC_ROOM_URL.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(PUBLIC_ROOM_URL);

        let headers = self.get_authorization_headers()?;

        let mut response = self.http_client.get(request_url.as_str()).headers(headers).send().map_err(MatrixClientError::Http)?;

        let mut body = String::new();
        response.read_to_string(&mut body).map_err(MatrixClientError::Io)?;

        if reqwest::StatusCode::Ok != *response.status() {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status(), body)));
        }

        let pub_rooms_response: PublicRoomsResponse = serde_json::from_str(&body).map_err(MatrixClientError::Json)?;

        Ok(pub_rooms_response)
    }

    pub fn join_room(&self, room_id_or_alias: &String) -> Result<JoinResponse, MatrixClientError> {
        if room_id_or_alias.is_empty() {
            return Err(MatrixClientError::BadRoomId(String::from("Room ID or Alias cannot be empty!")));
        }

        let mut request_url = String::with_capacity(self.homeserver.len() + JOIN_ROOM_URL.len() + room_id_or_alias.len());
        request_url.push_str(self.homeserver.as_str());
        request_url.push_str(JOIN_ROOM_URL);
        request_url.push_str(room_id_or_alias.as_str());

        let headers = self.get_authorization_headers()?;

        let mut response = self.http_client.post(request_url.as_str()).headers(headers).send().map_err(MatrixClientError::Http)?;

        let mut body = String::new();
        response.read_to_string(&mut body).map_err(MatrixClientError::Io)?;

        if reqwest::StatusCode::Ok != *response.status() {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status(), body)));
        }

        let join_response: JoinResponse = serde_json::from_str(&body).map_err(MatrixClientError::Json)?;

        Ok(join_response)
    }

    pub fn sync(&self, filter: Option<&str>, since: Option<&str>, full_state: Option<bool>, timeout: i64) -> Result<(), MatrixClientError> {
        let mut request_url = {
            let mut base_url = String::with_capacity(self.homeserver.len() + SYNC_URL.len());
            base_url.push_str(self.homeserver.as_str());
            base_url.push_str(SYNC_URL);
            reqwest::Url::parse(base_url.as_str()).map_err(MatrixClientError::UrlError)?
        };

        match filter {
            None => (),
            Some(filter_string) => {
                request_url.query_pairs_mut().append_pair("filter", filter_string);
            }
        };
        
        match since {
            None => (),
            Some(since_string) => {
                request_url.query_pairs_mut().append_pair("since", since_string);
            }
        };

        match full_state {
            None => (),
            Some(full_state_string) => {
                request_url.query_pairs_mut().append_pair("full_state", full_state_string.to_string().as_str());
            }
        };

        let headers = self.get_authorization_headers()?;

        let request = self.http_client.get(request_url).headers(headers);

        // FIXME: figure out how to set per-request timeouts
        //request.set_read_timeout(timeout).map_err(MatrixClientError::Http)?;

        let mut response = request.send().map_err(MatrixClientError::Http)?;

        let mut body = String::new();
        response.read_to_string(&mut body).map_err(MatrixClientError::Io)?;

        if reqwest::StatusCode::Ok != *response.status() {
            return Err(MatrixClientError::BadStatus(format!("Got error response from the server: {}; Contents: {}", response.status(), body)));
        }

        let sync_response: Option<SyncResponse> = match serde_json::from_str(&body) {
            Ok(x) => Some(x),
            Err(e) => {
                println!("Failed to deserialize! {:#?}", e);
                println!("{}", body);
                None
            }
        };

        match sync_response {
            Some(x) => println!("Got a sync response! {:#?}", x),
            _ => ()
        };

        Ok(())
    }
}

