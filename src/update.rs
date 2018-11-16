use discord_rpc_sdk::{RPC, RichPresence, EventHandlers};
use std::thread;
use float_duration::FloatDuration;
use std::time::SystemTime;
use std::{sync::{mpsc, Arc, Mutex, atomic::{AtomicBool, Ordering}}, rc::Rc};
use model::*;

struct AppClient {
    client: Option<RPC>,
    c_app_id: Option<u64>
}

static VALID_APP_ID: AtomicBool = AtomicBool::new(true);
impl AppClient {
    fn new() -> Self {
        AppClient {client: None, c_app_id: None}
    }

    fn new_client(&self, app_id: u64) -> bool {
        self.c_app_id != Some(app_id)
    }

    fn get_client(&mut self, app_id: u64) -> Option<&mut RPC> {
        if self.client.is_none() || Some(app_id) != self.c_app_id {
            self.client = None;

            VALID_APP_ID.store(true, Ordering::Relaxed);
            let client = RPC::init::<Self>(&app_id.to_string(), false, None).ok();

            self.c_app_id = Some(app_id);
            self.client = client;
        }

        self.client.as_mut()
    }
}

impl EventHandlers for AppClient {
    fn ready() {
        VALID_APP_ID.store(true, Ordering::Relaxed);
    }

    fn disconnected(errcode: i32, message: &str) {
        VALID_APP_ID.store(false, Ordering::Relaxed);
    }
}

fn str_opt(s: String, threshold: usize) -> Option<String> {
    if s.len() < threshold {
        None
    } else {
        Some(s)
    }
}

const DISCORD_CHAR_LIMIT: usize = 2 as usize;
pub fn start_updater(state: MState, wstate: WState) {
    let dur = FloatDuration::seconds(1.0);
    let stddur = dur.to_std().unwrap();
    let mut ac = AppClient::new();

    loop {
        thread::sleep(stddur);

        let presence = {
            let mut s = state.lock().unwrap();

            let cur = s.current;
            let p: Option<&mut Presence> = s.presences.get_mut(cur);
            if let Some(presence) = p {
                if let Some(&TimePresence { length: Some(length), .. }) = presence.current() {
                    presence.elapsed += dur;

                    if presence.elapsed > length {
                        presence.active_presence = (presence.active_presence + 1) % presence.time_presences.len();
                        presence.elapsed = FloatDuration::zero();
                    }
                }

                presence.current().cloned().map(|x| (presence.clone(), x))
            } else { None }
        };

        if let Some((pres, tpres)) = presence {
            let mut app_res = ac.get_client(pres.application_id);

            if let Some(mut client) = app_res {
                let (start_time, end_time) = match tpres.show_time {
                    ShowTime::Elapsed | ShowTime::Remaining => {
                        let start = SystemTime::now() - pres.elapsed.to_std().unwrap();

                        let end = match (tpres.show_time, tpres.length) {
                            (ShowTime::Remaining, Some(dur)) => {
                                Some(start + dur.to_std().unwrap())
                            },
                            _ => None
                        };

                        (Some(start), end)
                    },
                    _ => (None, None)
                };

                let l = pres.time_presences.len();
                let (party_size, party_max) = if l > 1 {
                    (Some((pres.active_presence+1) as u32), Some(l as u32))
                } else { (None, None) };

                let _ = client.update_presence(RichPresence {
                    details: str_opt(tpres.details, DISCORD_CHAR_LIMIT),
                    state: str_opt(tpres.state, DISCORD_CHAR_LIMIT),
                    start_time,
                    end_time,

                    large_image_key: Some(tpres.large_image),
                    small_image_key: Some(tpres.small_image),
                    small_image_text: str_opt(tpres.tooltip.clone(), DISCORD_CHAR_LIMIT),
                    large_image_text: str_opt(tpres.tooltip, DISCORD_CHAR_LIMIT),

                    party_id: None,
                    party_size,
                    party_max,
                    spectate_secret: None,
                    join_secret: None
                }).map_err(|x| println!("ERROR: {:?}", x));

                client.run_callbacks();
            }

            if let WinState::Open(ref pcfg) = *wstate.lock().unwrap() {
                pcfg.call_method("worker_update",
                                 &[(pres.active_presence as i32).into(), ::value::dur_val(pres.elapsed),
                                     VALID_APP_ID.load(Ordering::Relaxed).into()]).unwrap();
            };
        }
    }
}
