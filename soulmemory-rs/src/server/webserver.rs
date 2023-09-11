use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::mem;
use rouille::Response;
use rouille::router;
use std::thread::{Builder};
use crate::games::EventFlag;
use serde::Serialize;
use serde::Deserialize;

pub struct Webserver
{
    event_flag_states: HashMap<u32, bool>,
    event_flag_buffer: Arc<Mutex<Vec<EventFlag>>>,
}

impl Webserver
{
    pub fn new() -> Self
    {
        let webserver = Webserver
        {
            event_flag_states: HashMap::new(),
            event_flag_buffer: Arc::new(Mutex::new(Vec::new())),
        };

        let event_flags = Arc::clone(&webserver.event_flag_buffer);
        Builder::new().name("webserver_thread".to_string()).spawn(move || {
            rouille::start_server("127.0.0.1:2345", move |request|
            {
                return  router!(request,
                    (GET) (/event_flags) =>
                    {
                        let mut guard_event_flags = event_flags.lock().unwrap();
                        let clone = guard_event_flags.clone();
                        guard_event_flags.clear();
                        return Response::text(serde_json::to_string(&clone).unwrap());
                    },
                    _ => Response::text("hello world")
                );
            });
        }).expect("Webserver error");

        return webserver;
    }

    pub fn add_flag(&mut self, event_flag: EventFlag)
    {
        let mut event_flags = self.event_flag_buffer.lock().unwrap();

        if !self.event_flag_states.contains_key(&event_flag.flag)
        {
            self.event_flag_states.insert(event_flag.flag, event_flag.state);
            event_flags.push(event_flag);
            return;
        }

        if let Some(current_state) = self.event_flag_states.get_mut(&event_flag.flag)
        {
            if *current_state != event_flag.state
            {
                *current_state = event_flag.state;
                event_flags.push(event_flag);
                return;
            }
        }
    }
}