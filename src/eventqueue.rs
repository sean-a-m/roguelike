use uuid::Uuid;
use std::collections::BTreeSet;

pub struct event_list {
    pub e_list: Vec<event_item>,
}

pub struct event_item {
        pub e_time: i64,
        pub material: Uuid,
}