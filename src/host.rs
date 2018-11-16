use model::*;
use sciter::*;
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct Handler {
    pub host: Rc<Host>,
    pub state: MState,
    pub wstate: WState
}

fn parse_arg<T: FromValue>(x: &Value) -> Res<T> {
    T::from_value(x).ok_or("Wrong argument type!".into())
}

fn handle_script_call(state: MState, wstate: WState, name: &str, args: &[Value]) -> Res<Option<Value>> {
    let mut s = state.lock().unwrap();

    let ok: Value = true.into();

    match (name, args) {
        ("add_presence", [presence]) => {
            let p = if presence.is_null() { Presence::new(&s) }
                else { parse_arg(presence)? };

            s.presences.push(p.clone());

            let mut ret = Value::array(0);
            ret.push(p); ret.push((s.presences.len() as i32)-1);

            Ok(Some(ret))
        },

        ("del_presence", [i]) => {
            let i= parse_arg::<i32>(i)? as usize;
            s.presences.remove(i);

            Ok(Some(ok))
        },

        ("rename_presence", [i, new_name]) => {
            let i= parse_arg::<i32>(i)? as usize;
            let name = parse_arg::<String>(new_name)?;

            if name.len() > 16 {
                return Err("Presence name cannot be over 16 characters!".into());
            } else if name.len() == 0 {
                return Err("Presence name cannot be empty!".into());
            }

            s.presences[i].name = name;
            Ok(Some(ok))
        },

        ("ser_presence", [i]) => {
            let i = parse_arg::<i32>(i)? as usize;
            Ok(Some(serde_json::to_string_pretty(&s.presences[i])?.into()))
        },

        ("de_presence", [presence]) => {
            let json_str = parse_arg::<String>(presence)?;
            let p: Presence = serde_json::from_str(&json_str).map_err(|_| "Invalid json")?;

            Ok(Some(p.into()))
        }

        ("reorder_presence", [i1, i2]) => {
            let i1 = parse_arg::<i32>(i1)? as usize;
            let i2 = parse_arg::<i32>(i2)? as usize;

            let e = s.presences.remove(i1);
            s.presences.insert(i2, e);
            Ok(Some(ok))
        },

        ("activate_presence", [i]) => {
            let i= parse_arg::<i32>(i)? as usize;
            s.current = i;

            Ok(Some(s.presences[i].clone().into()))
        },

        ("update_presence", [p]) => {
            let cur = s.current;
            s.presences[cur] = Presence::from_value(p).ok_or("Error deserializing presence")?;
            Ok(Some(ok))
        },

        ("new_time_presence", []) => {
            Ok(Some(TimePresence::new().into()))
        },

        ("win", [new_state]) => {
            let new_state = parse_arg::<String>(new_state)?;
            match new_state.as_str() {
                "closing" => *wstate.lock().unwrap() = WinState::Closing,
                _ => ()
            }

            Ok(Some(ok))
        },

        _ => Ok(None)
    }
}

impl EventHandler for Handler {
    fn on_script_call(&mut self, root: HELEMENT, name: &str, args: &[Value]) -> Option<Value> {
        match handle_script_call(self.state.clone(), self.wstate.clone(), name, args) {
            Ok(x) => {
                x.map(|x| {
                    let mut v = Value::new();
                    v.push(Value::symbol("ok"));
                    v.push(x);

                    v
                })
            }, Err(x) => {
                let arg = Value::from(x.description());
                let mut v = Value::new();
                v.push(Value::symbol("error")); v.push(arg);

                Some(v)
            }
        }
    }
}