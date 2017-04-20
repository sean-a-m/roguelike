extern crate rustty;

use rustty::{Terminal, Cell, HasSize, CellAccessor, Event};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};
use std::time::Duration;
use std::f64;

trait Visible {
    fn visibilize(&self) -> char;
}

trait Can_Draw {
    fn draw(&self, &mut Widget, &Vec<Substance>) -> ();
}

struct Substance {
    x: i64,
    y: i64,
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

impl Can_Draw for Viewport {
    fn draw(&self, canvas: &mut Widget, s_list: &Vec<Substance>) -> () {
        for s in s_list.into_iter().filter(|&i| (i.x - self.x >= 0 && i.y - self.y >= 0)) {
            //TODO: find out what actually happens when you cast values like this in rust  
            canvas.get_mut((s.x - self.x) as usize, (s.y - self.y) as usize).unwrap().set_ch(s.glyph);
        }
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
                                layer: 1 };
    let viewport = Viewport {x: 0, y: 0, xs: term.size().0, ys: term.size().1};
    let mut visibles = Vec::new(); 
    visibles.push(test_wall);
    canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);
    canvas.get_mut(5,5).unwrap().set_ch('x');
    viewport.draw(&mut canvas, &visibles);
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
