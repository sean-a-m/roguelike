extern crate rustty;
extern crate uuid;
extern crate ndarray;

mod viewport;
mod substance;
mod grid;
mod eventqueue;

use rustty::{Terminal, Cell as TTYCell, HasSize, CellAccessor, Event};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};
use std::time::Duration;
use std::f32;
use std::collections::HashMap;
use std::cell;
use uuid::Uuid;
use ndarray::{arr2,Array,Array2};
use substance::{Substance, Entity};
use grid::OccupancyGrid;
use viewport::{Viewport, Can_Draw};


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
