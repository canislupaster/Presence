use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Clone, Hash, PartialEq, Debug)]
/// Defines the data displayed on the rich presence screen on a user's profile.
pub struct RichPresence {
    /// The user's current party status. Maximum of 128 bytes.
    ///
    /// For example: `"Looking to Play"`, `"Playing Solo"`, `"In a Group"`...
    pub state: Option<String>,

    /// What the player is currently doing. Maximum of 128 bytes.
    ///
    /// For example: `"Competitive - Captain's Mode"`, `"In Queue"`, `"Unranked PvP"`...
    pub details: Option<String>,

    /// Time of game start. Including will show time as "elapsed".
    pub start_time: Option<SystemTime>,

    /// Time of game end. Including will show time as "remaining".
    pub end_time: Option<SystemTime>,

    /// Name of the uploaded image for the large profile artwork. Maximum of 32 bytes.
    pub large_image_key: Option<String>,

    /// Tooltip for the large image. Maximum of 128 bytes.
    pub large_image_text: Option<String>,

    /// Name of the uploaded image for the large profile artwork. Maximum of 32 bytes.
    pub small_image_key: Option<String>,

    /// Tooltip for the large image. Maximum of 128 bytes.
    pub small_image_text: Option<String>,

    /// ID of the player's party, lobby, or group. Maximum of 128 bytes.
    pub party_id: Option<String>,

    /// Current size of the player's party, lobby, or group.
    pub party_size: Option<u32>,

    /// Maximum size of the player's party, lobby, or group.
    pub party_max: Option<u32>,

    /// Unique hashed string for Spectate button. Maximum of 128 bytes.
    pub spectate_secret: Option<String>,

    /// Unique hashed string for chat invitations and Ask to Join. Maximum of 128 bytes.
    pub join_secret: Option<String>,
}

use sys;
use libc;
use std::ffi::{CString, NulError};
use std::ptr;

impl RichPresence {
    pub(crate) fn wrap(self) -> Result<sys::DiscordRichPresence, NulError> {
        Ok(sys::DiscordRichPresence {
            state: match self.state {
                None => ptr::null(),
                Some(state) => CString::new(state.clone())?.into_raw(),
            },
            details: match self.details {
                None => ptr::null(),
                Some(details) => CString::new(details)?.into_raw(),
            },
            startTimestamp: match self.start_time {
                None => 0,
                Some(time) => time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            },
            endTimestamp: match self.end_time {
                None => 0,
                Some(time) => time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            },
            largeImageKey: match self.large_image_key {
                None => ptr::null(),
                Some(key) => CString::new(key)?.into_raw(),
            },
            largeImageText: match self.large_image_text {
                None => ptr::null(),
                Some(text) => CString::new(text)?.into_raw(),
            },
            smallImageKey: match self.small_image_key {
                None => ptr::null(),
                Some(key) => CString::new(key)?.into_raw(),
            },
            smallImageText: match self.small_image_text {
                None => ptr::null(),
                Some(text) => CString::new(text)?.into_raw(),
            },
            partyId: match self.party_id {
                None => ptr::null(),
                Some(id) => CString::new(id)?.into_raw(),
            },
            partySize: match self.party_size {
                None => 0,
                Some(size) => size as libc::c_int,
            },
            partyMax: match self.party_max {
                None => 0,
                Some(max) => max as libc::c_int,
            },
            matchSecret: ptr::null(), // deprecated
            joinSecret: match self.join_secret {
                None => ptr::null(),
                Some(secret) => CString::new(secret)?.into_raw(),
            },
            spectateSecret: match self.spectate_secret {
                None => ptr::null(),
                Some(secret) => CString::new(secret)?.into_raw(),
            },
            instance: 0, // deprecated
        })
    }
}
