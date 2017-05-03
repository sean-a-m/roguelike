
extern crate rustty;
extern crate uuid;

use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};
use rustty::{Terminal, Cell as TTYCell, HasSize, CellAccessor, Event};
use std::collections::HashMap;
use uuid::Uuid;
use substance::{Substance, Entity};
use grid::OccupancyGrid;


    pub struct Viewport {
        pub x: i32,
        pub y: i32,
        pub xs: usize,
        pub ys: usize,
    }

    pub trait Can_Draw {
        fn draw(&self, &mut Widget, &HashMap<Uuid, Substance>) -> ();
        fn draw_occupants(&self, &mut Widget, &OccupancyGrid, &Viewport);
    }

    impl Can_Draw for Viewport {
        fn draw(&self, canvas: &mut Widget, s_list: &HashMap<Uuid, Substance>) -> () {
            for (uuid, s) in s_list {
                if s.x.get() - self.x >= 0 && s.y.get() - self.y >= 0 {
                //TODO: find out what actually happens when you cast values like this in rust  
                    canvas.get_mut((s.x.get() - self.x) as usize, (s.y.get() - self.y) as usize).unwrap().set_ch(s.glyph);
                }
            }
        }

        fn draw_occupants(&self, canvas: &mut Widget, occupancy_grid: &OccupancyGrid, viewport: &Viewport) {
            for x in viewport.x..(viewport.x + viewport.xs as i32 + 1) {
                for y in viewport.x..(viewport.x + viewport.xs as i32 + 1) {
                    let occupants = occupancy_grid.getOccupants(x, y);

                }
            }
        }
    }
