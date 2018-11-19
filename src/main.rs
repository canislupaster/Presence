#![feature(windows_subsystem)]
#![windows_subsystem = "windows"]

#[macro_use]
extern crate sciter;
extern crate systray;
extern crate discord_rpc_sdk;
extern crate libc;

extern crate float_duration;
#[macro_use]
extern crate lazy_static;
//TODO actually use rayon
extern crate rayon;
extern crate serde;
extern crate serde_derive;
extern crate dirs;

use std::fs;
use fs::File;
use std::thread;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use sciter::{host as shost, window::Options, utf::w2s, Value, HELEMENT, EventHandler, HostHandler};

pub mod model;
pub mod update;
pub mod host;
pub mod tray;
pub mod value;

use model::*;

struct MainHostHandler {
    assets: sciter::Archive
}

impl HostHandler for MainHostHandler {
    fn on_data_load(&mut self, pnm: &mut shost::SCN_LOAD_DATA) -> Option<shost::LOAD_RESULT> {
        let uri = &w2s(pnm.uri);
        let filepath: PathBuf = uri["file://".to_string().len()..].to_owned().into();

        self.assets.get(&filepath.to_string_lossy()).map(|data| {
            self.data_ready(pnm.hwnd, &uri, data, None);
            shost::LOAD_RESULT::LOAD_DEFAULT
        })
    }
}

#[cfg(debug_assertions)]
struct DebugHostHandler;

#[cfg(debug_assertions)]
impl HostHandler for DebugHostHandler {
    fn on_data_load(&mut self, pnm: &mut shost::SCN_LOAD_DATA) -> Option<shost::LOAD_RESULT> {
        let uri = &w2s(pnm.uri);

        let mut path = std::env::current_dir().unwrap();
        path.push("res");

        let filepath: PathBuf = uri["file://".to_string().len()..].to_owned().into();

        path.push(filepath);
        if let Ok(b) = fs::read(path) {
            self.data_ready(pnm.hwnd, &uri, &b.as_slice(), None);
            return Some(shost::LOAD_RESULT::LOAD_DEFAULT);
        }

        None
    }
}

#[cfg(debug_assertions)]
fn setup(win: &mut sciter::Window) {
    win.sciter_handler(DebugHostHandler);
    win.load_file("file://main.html");
}

#[cfg(not(debug_assertions))]
fn setup(win: &mut sciter::Window) {
    let archive = include_bytes!("./resources.rc");
    let assets = shost::Archive::open(archive).expect("Error loading archive!");

    win.sciter_handler(MainHostHandler {assets});
    win.load_file("file://main.html");
}

fn start_win(mstate: MState) -> sciter::Window {
    let mut win = sciter::WindowBuilder::main_window().resizeable().alpha().glassy().with_size((900, 800)).create();
    let opts = vec![Options::TransparentWindow(true), Options::AlphaWindow(true), Options::DebugMode(cfg!(debug_assertions))];

    opts.iter().for_each(|opt| win.set_options(opt.clone()).expect("Error setting debugmode!"));

    win.set_title("Presence");

    let host = win.get_host();
    let handler = host::Handler {host: host.clone(), state: mstate.clone(), wstate: WINSTATE.clone() };
    win.event_handler(handler);
    setup(&mut win);

    let _ = win.get_host().call_function("load", &[mstate.lock().unwrap().clone().into()]);
    win
}

lazy_static! {
    static ref WINSTATE: WState = Arc::new(Mutex::new(WinState::Opening));
}

fn main() {
    let mut cfgpath = dirs::config_dir().expect("Error finding configuration dir");
    cfgpath.push("presence_config.json");

    let state = {
        if let Some(x) = fs::read_to_string(&cfgpath).ok()
            .and_then(|x| serde_json::from_str(&x).ok()) {

            x
        } else {
            State::new()
        }
    };

    let mstate = Arc::new(Mutex::new(state));

    thread::spawn({
        let mstate = mstate.clone();
        let winstate = WINSTATE.clone();

        move || update::start_updater(mstate, winstate)
    });

    loop {
        let ws = WINSTATE.lock().unwrap().clone();
        match ws {
            WinState::Opening => {
                let win = start_win(mstate.clone());
                let pcfg = win.get_host().get_root().unwrap().find_first("#container").unwrap().unwrap();

                *WINSTATE.lock().unwrap() = WinState::Open(pcfg);
                win.run_app();
            },

            WinState::Closing => break,
            _ => thread::sleep_ms(100)
        }
    }

    let s = &*mstate.lock().unwrap();
    fs::write(&cfgpath, serde_json::to_string(s).expect("Error serializing config")).expect("Error saving config");
}
