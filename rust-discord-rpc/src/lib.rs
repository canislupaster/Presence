extern crate discord_rpc_sys as sys;
extern crate libc;

mod join_request;
pub use join_request::{JoinRequest, JoinRequestReply};

mod presence;
pub use presence::RichPresence;

mod event_handlers;
pub use event_handlers::EventHandlers;

mod event_wrappers;

pub struct RPC;

use std::ffi::{CString, NulError};
use std::ptr;

impl RPC {
    /// Initializes the RPC API.
    pub fn init<EH: EventHandlers>(
        app_id: &str,
        auto_register: bool,
        steam_id: Option<&str>,
    ) -> Result<RPC, NulError> {
        let mut sys_handlers = event_handlers::wrap::<EH>();
        unsafe {
            sys::Discord_Initialize(
                CString::new(app_id)?.into_raw(),
                &mut sys_handlers,
                auto_register as libc::c_int,
                match steam_id {
                    None => ptr::null(),
                    Some(id) => CString::new(id)?.into_raw(),
                },
            );
        }

        Ok(RPC)
    }

    /// Updates the callback handlers.
    pub fn update_handlers<EH: EventHandlers>(&self) {
        let mut sys_handlers = event_handlers::wrap::<EH>();
        unsafe {
            sys::Discord_UpdateHandlers(&mut sys_handlers);
        }
    }

    /// Updates the rich presence screen.
    pub fn update_presence(&self, presence: RichPresence) -> Result<(), NulError> {
        let sys_presence = presence.wrap()?;
        unsafe {
            sys::Discord_UpdatePresence(&sys_presence);
        }

        Ok(())
    }

    /// Clears the rich present screen.
    pub fn clear_presence(&self) {
        unsafe {
            sys::Discord_ClearPresence();
        }
    }

    /// Invokes any pending callbacks from Discord on the calling thread. This
    /// function is allegedly thread safe.
    pub fn run_callbacks(&self) {
        unsafe {
            sys::Discord_RunCallbacks();
        }
    }
}

impl Drop for RPC {
    fn drop(&mut self) {
        unsafe {
            sys::Discord_Shutdown();
        }
    }
}
