extern crate uuid;
use uuid::Uuid;
use std::cell;



    #[derive(Clone)]
    pub struct Substance {
        pub handle: Uuid,
        pub x: cell::Cell<i32>,
        pub y: cell::Cell<i32>,
        pub move_cost: f32,
        pub glyph: char,
        pub layer: i8,
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
