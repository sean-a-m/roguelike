use ndarray::{arr2,Array,Array2};

use viewport::Viewport;
use substance::Substance;
use uuid::Uuid;

pub struct OccupancyGrid {
    pub grid: Array2<Vec<Uuid>>,
}

impl OccupancyGrid{
    pub fn getOccupants(&self, x: i32, y: i32) -> Vec<Uuid> {
        //TODO: add error handling here
        match self.grid.get((x as usize, y as usize)) {
            Some(ids) => ids.clone(),
            None => Vec::new(),
        }
    }

    //TODO: Return some sort of success/failure indicator
    pub fn addOccupant(&mut self, x: usize, y: usize, id: Uuid) -> () {
        //TODO: add error handling here
        let ids = self.grid.get_mut((x,y)).unwrap();
        ids.push(id);
    }

    //TODO: Return some sort of success/failure indicator
    pub fn removeOccupant(&mut self, x: usize, y: usize, id: Uuid) -> () {
        //TODO: add error handling here
        let ids = self.grid.get_mut((x,y)).unwrap();
        let index = ids.iter().position(|x| *x == id).unwrap();
        ids.remove(index);
    }
}




