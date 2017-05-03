extern crate rustty;
extern crate uuid;
extern crate ndarray;

use rustty::{Terminal, Cell as TTYCell, HasSize, CellAccessor, Event};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};
use std::time::Duration;
use std::f32;
use std::collections::HashMap;
use std::cell;
use uuid::Uuid;
use ndarray::{arr2,Array,Array2};

trait Visible {
    fn visibilize(&self) -> char;
}

trait Can_Draw {
    fn draw(&self, &mut Widget, &HashMap<Uuid, Substance>) -> ();
    fn draw_occupants(&self, &mut Widget, &OccupancyGrid, &Viewport);
}

trait Entity {
    fn get_handle(&self) -> Uuid;
}

#[derive(Clone)]
struct Substance {
    handle: Uuid,
    x: cell::Cell<i32>,
    y: cell::Cell<i32>,
    move_cost: f32,
    glyph: char,
    layer: i8,
}

struct Viewport {
    x: i32,
    y: i32,
    xs: usize,
    ys: usize,
}

struct OccupancyGrid {
    grid: Array2<Vec<Uuid>>,
}

impl OccupancyGrid{
    fn getOccupants(&self, x: i32, y: i32) -> Vec<Uuid> {
        //TODO: add error handling here
        match self.grid.get((x as usize, y as usize)) {
            Some(ids) => ids.clone(),
            None => Vec::new(),
        }
    }

    //TODO: Return some sort of success/failure indicator
    fn addOccupant(&mut self, x: usize, y: usize, id: Uuid) -> () {
        //TODO: add error handling here
        let ids = self.grid.get_mut((x,y)).unwrap();
        ids.push(id);
    }

    //TODO: Return some sort of success/failure indicator
    fn removeOccupant(&mut self, x: usize, y: usize, id: Uuid) -> () {
        //TODO: add error handling here
        let ids = self.grid.get_mut((x,y)).unwrap();
        let index = ids.iter().position(|x| *x == id).unwrap();
        ids.remove(index);
    }
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

    fn draw_occupants(&self, canvas: &mut Widget, occupancy_grid: &OccupancyGrid, viewport: &Viewport) {
        for x in viewport.x..(viewport.x + viewport.xs as i32 + 1) {
            for y in viewport.x..(viewport.x + viewport.xs as i32 + 1) {
                let occupants = occupancy_grid.getOccupants(x, y);

            }
        }
    }
}

fn move_occupant(occupancy_grid: &mut OccupancyGrid, materials_dict: &HashMap<Uuid, Substance>, id: Uuid, x: usize, y: usize) -> () {
    let material = materials_dict.get(&id).unwrap();
    let old_x = material.x.get();
    let old_y = material.y.get();
    occupancy_grid.removeOccupant(old_x as usize, old_y as usize, id);
    occupancy_grid.addOccupant(x, y, id);
}

fn get_rect(substance: &Substance, x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<Substance> {
    let mut additions = Vec::new();
    for x in x0..x1 {
        for y in y0..y1 {
            let new_substance = Substance{ handle: Uuid::new_v4(),
                                           x: cell::Cell::new(x),
                                           y: cell::Cell::new(y),
                                           move_cost: substance.move_cost,
                                           glyph: substance.glyph,
                                           layer: substance.layer };
            additions.push(new_substance);
        }
    }
    return additions
}

fn get_hline(substance: &Substance, x0: i32, x1: i32, y: i32) -> Vec<Substance> {
    let mut additions = Vec::new();
    for x in x0..x1 {
            let new_substance = Substance{ handle: Uuid::new_v4(),
                                           x: cell::Cell::new(x),
                                           y: cell::Cell::new(y),
                                           move_cost: substance.move_cost,
                                           glyph: substance.glyph,
                                           layer: substance.layer };
            additions.push(new_substance);
    }
    return additions
}

fn get_vline(substance: &Substance, y0: i32, y1: i32, x: i32) -> Vec<Substance> {
    let mut additions = Vec::new();
        for y in y0..y1 {
            let new_substance = Substance{ handle: Uuid::new_v4(),
                                           x: cell::Cell::new(x),
                                           y: cell::Cell::new(y),
                                           move_cost: substance.move_cost,
                                           glyph: substance.glyph,
                                           layer: substance.layer };
            additions.push(new_substance);
    }
    return additions
}

fn main() {

    let mut term = Terminal::new().unwrap();
    let mut canvas = Widget::new(term.size().0, term.size().1);
    let mut substance_dict = HashMap::new();
    let wall_template = Substance { handle: Uuid::new_v4(),
                                x: cell::Cell::new(8), 
                                y: cell::Cell::new(8), 
                                move_cost: f32:: INFINITY,
                                glyph: 'O',
                                layer: 1 };

    let floor_template = Substance { handle: Uuid::new_v4(),
    x: cell::Cell::new(8), 
    y: cell::Cell::new(8), 
    move_cost: 1.0,
    glyph: '+',
    layer: 1 };

    let av = Substance { handle: Uuid::new_v4(),
                        x: cell::Cell::new(2), 
                        y: cell::Cell::new(2), 
                        move_cost: f32:: INFINITY,
                        glyph: '@',
                        layer: 1 };

    //Extend keeps the old copies of these around but they should go out of scope anyway so I don't think it matters
    let mut map = get_rect(&floor_template, 1, 1, 5, 5);
                map.extend(get_hline(&wall_template, 0, 5, 0));
                map.extend(get_hline(&wall_template, 0, 5, 5));
                map.extend(get_vline(&wall_template, 0, 5, 0));
                map.extend(get_vline(&wall_template, 0, 5, 5));


    let mut occupancy_grid = OccupancyGrid { grid: Array2::<Vec<Uuid>>::default((50, 50)) };

    for item in map {
        occupancy_grid.addOccupant(item.x.get() as usize, item.y.get() as usize, item.handle);
        substance_dict.insert(item.get_handle(), item);
    }

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
