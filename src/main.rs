extern crate rustty;

use rustty::{Terminal, Cell, HasSize, CellAccessor, Event};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};
use std::time::Duration;
use std::f64;

trait Visible {
    fn visibilize(&self) -> char;
}

struct Substance {
    x: u64,
    y: u64,
    move_cost: f64,
    glyph: char,
    layer: i8,
}


impl Visible for Substance {
    fn visibilize(&self) -> char {
        self.glyph
    }
}

fn main() {
    println!("1.0 * 5.0 = {}", 1.0 * 5.0);
    println!("2.2 * 3.2 = {}", 2.2 * 3.2);
    println!("3.0 / INF = {}", 3.0f64 / f64::INFINITY);
    println!("3.0 * INF = {}", 3.0f64 * f64::INFINITY);
    let mut term = Terminal::new().unwrap();
    let mut canvas = Widget::new(term.size().0, term.size().1);
    let test_wall = Substance { x: 8, 
                                y: 8, 
                                move_cost: f64:: INFINITY,
                                glyph: 'O',
                                layer: 1};
    canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);
    canvas.get_mut(5,5).unwrap().set_ch('x');
    canvas.get_mut(test_wall.x as usize, test_wall.y as usize).unwrap().set_ch(test_wall.glyph);
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
