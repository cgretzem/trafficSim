use crate::traffic_logic::car::Direction;

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Node{
    pub dest_int_id:u8,
    pub dist_from_source:u8,
    pub direction:u8
}

impl Node{
    pub fn new(dest_int_id:u8, dist_from_source:u8, direction:u8) -> Node{
        Node{dest_int_id, dist_from_source, direction}
    }
}


pub struct Road
{
    ///Represents a weighted graph between intersections, hashmap data structure which
    /// hashes an intersection ID to a list of Tuples which contain : 
    /// # Tuple Members
    /// * `Destination Intersection ID` : The ID of the intersection
    /// * `Distance from Source` : The number of ticks it takes to get to the destination from the source
    /// * `Direction` : The direction North South East West corresponding to 0,1,2,3 that car will arrive at
    pub road : HashMap<u8, [Option<Node>;4]> 

}


impl Road
{
    ///Creates a new Road
    /// # Returns
    /// `Road`
    pub fn new() -> Road
    {
        Road{road:HashMap::new()}
    }

    pub fn get_distance(&self, source : u8, dest : u8) -> Option<u8>
    {
        let distance = self.road.get(&source)?
        .iter()
        .find(|opt| {
            match opt{
                None => false,
                Some(tup) => tup.dest_int_id == dest
            }
        }).unwrap_or_else(|| panic!("Intersection {} has no connection to Intersection {}", source, dest)).unwrap().dist_from_source;
        Some(distance)
    }

    ///Gets the next Intersection given a source intersection, inital direction, and intent
    /// # Parameters
    /// * `source` : `u8` - The IntersectionID of the source intersection
    /// * `source_dir` : `u8` The inital 0,1,2,3 Direction of the car
    /// * `direction` : `Direction` - The direction intention of the car
    /// # Returns
    /// `Option<(u8,u8)>` : Returns `None` if no Intersection was found
    /// * `next_intersection_id` : `u8` - The IntersectionID of the next intersection
    /// * `next_direction` : `u8` - The direction the car will arrive at
    pub fn get_next_intersection(&self, source : u8, source_dir : u8, direction : Direction) -> Option<(u8, u8)>
    {
        let index : usize = match direction
        {
            Direction::Straight => usize::from(source_dir+2)%4,
            Direction::Left => usize::from(source_dir+1)%4,
            Direction::Right => usize::from(source_dir+3)%4,
            _=> usize::from(source_dir)
        };
        let next = self.road.get(&source)?[index]?;
        Some((next.dest_int_id, next.direction))

    }
}

