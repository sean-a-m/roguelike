extern crate uuid;
use uuid::Uuid;
use std::cell;
use grid::OccupancyGrid;
use std::collections::HashMap;
use eventqueue::event_item;




    #[derive(Clone)]
    pub struct Substance {
        pub handle: Uuid,
        pub x: cell::Cell<i32>,
        pub y: cell::Cell<i32>,
        pub move_cost: f32,
        pub glyph: char,
        pub layer: i8,
    }

    #[derive(Clone)]
    pub struct AutonomousSubstance {
        pub handle: Uuid,
        pub x: cell::Cell<i32>,
        pub y: cell::Cell<i32>,
        pub move_cost: f32,
        pub glyph: char,
        pub layer: i8,
    }

    trait Autonomous {
        fn autonomize(&self, occupants: &OccupancyGrid, materials: &HashMap<Uuid, Substance>) -> event_item;
    }

    trait Visible {
        fn visibilize(&self) -> char;
    }

    pub trait Entity {
        fn get_handle(&self) -> Uuid;
    }

    impl Visible for Substance {
        fn visibilize(&self) -> char {
            self.glyph
        }
    }

    impl Entity for Substance {
        fn get_handle(&self) -> Uuid {
            self.handle
        }
    }

    impl Visible for AutonomousSubstance {
        fn visibilize(&self) -> char {
            self.glyph
        }
    }

    impl Entity for AutonomousSubstance {
        fn get_handle(&self) -> Uuid {
            self.handle
        }
    }

    impl AutonomousSubstance {
        fn autonomize(&self, occupants: &OccupancyGrid, materials: &HashMap<Uuid, Substance>) -> event_item {
            event_item {
                e_time: 1,
                material: Uuid::new_v4(),
            }
        }
    }
