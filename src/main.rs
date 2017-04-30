extern crate rustty;
extern crate uuid;

use rustty::{Terminal, Cell, HasSize, CellAccessor, Event};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};
use std::time::Duration;
use std::f64;
use std::collections::HashMap;
use std::cell;
use uuid::Uuid;

trait Visible {
    fn visibilize(&self) -> char;
}

trait Can_Draw {
    fn draw(&self, &mut Widget, &HashMap<Uuid, Substance>) -> ();
}

trait Entity {
    fn get_handle(&self) -> Uuid;
}

struct Substance {
    handle: Uuid,
    x: cell::Cell<i64>,
    y: cell::Cell<i64>,
    move_cost: f64,
    glyph: char,
    layer: i8,
}

struct Viewport {
    x: i64,
    y: i64,
    xs: usize,
    ys: usize,
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

impl Can_Draw for Viewport {
    fn draw(&self, canvas: &mut Widget, s_list: &HashMap<Uuid, Substance>) -> () {
        for (uuid, s) in s_list {
            if s.x.get() - self.x >= 0 && s.y.get() - self.y >= 0 {
            //TODO: find out what actually happens when you cast values like this in rust  
                canvas.get_mut((s.x.get() - self.x) as usize, (s.y.get() - self.y) as usize).unwrap().set_ch(s.glyph);
            }
        }
    }
}


fn main() {

    let mut term = Terminal::new().unwrap();
    let mut canvas = Widget::new(term.size().0, term.size().1);
    let mut substance_dict = HashMap::new();
    let test_wall = Substance { handle: Uuid::new_v4(),
                                x: cell::Cell::new(8), 
                                y: cell::Cell::new(8), 
                                move_cost: f64:: INFINITY,
                                glyph: 'O',
                                layer: 1 };

    let av = Substance { handle: Uuid::new_v4(),
                        x: cell::Cell::new(5), 
                        y: cell::Cell::new(5), 
                        move_cost: f64:: INFINITY,
                        glyph: '@',
                        layer: 1 };

    substance_dict.insert(test_wall.get_handle(), test_wall);
    substance_dict.insert(av.get_handle(), av);
    
    let viewport = Viewport {x: 0, y: 0, xs: term.size().0, ys: term.size().1};
    canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);
    canvas.get_mut(5,5).unwrap().set_ch('x');
    viewport.draw(&mut canvas, &substance_dict);
    canvas.draw_into(&mut term);    
    term.swap_buffers().unwrap();
    


    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(Some(Duration::new(0, 0)).unwrap()).unwrap() {
            match ch {
                'q' => break 'main,
                _ => {}
            }
        }
    }

}
