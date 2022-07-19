use crate::traffic_logic::{road::Road, car::{Car, Direction}, intersection::Intersection};
use std::{collections::HashMap};


pub struct Position
{
    ///an Option of a tuple, with tuple.0 being the intersectionID 
    /// and tuple.1 being the true direction of the car, IE which direction it is wating at. 
    /// # Tuple Objects
    /// * `IntersectionID` - the ID of the intersection the car is currently at
    /// * `Direction ID` - a u8 representing a direction
    /// # Directions
    /// * 0=>north 
    /// * 1=>east 
    /// * 2=>south 
    /// * 3=>west
    pub current_intersection: Option<(u8, u8)>,

    ///An option of a tuple with the following members
    /// # Members
    /// * `Intersection_1_ID` : `u8` - The ID of the intersection the car is coming from 
    /// * `Intersection_2_ID` : `u8` - The ID of the intersection the car is going to
    /// * `current_distance_to_target` : `u8` - the current distance from the destination Intersection
    /// * `from_direction` : `u8 - The direction the car is going 0,1,2,3
    pub in_between : Option<(u8, u8, u8, u8)>
}

impl Position
{
    fn new(current_intersection : Option<(u8, u8)>, in_between : Option<(u8, u8, u8, u8)>) -> Position
    {
        Position { current_intersection, in_between}
    }
    
    fn get_distance(&self) -> u8
    {
        if let None = self.in_between
        {
            panic!("Cannot call get distance when car is not between 2 intersections");
        }
        self.in_between.as_ref().unwrap().2
    }
}

pub struct Simulator
{
    road:Road,
    car_positions: HashMap<u8, Position>,
    cars : Vec<Car>,
    intersections : Vec<Intersection>,
    timestep: usize

}

impl Simulator
{
    fn new(road:Road) -> Simulator
    {
        Simulator{road, car_positions: HashMap::new(), timestep:0, cars:Vec::new(), intersections:Vec::new()}
    }

    fn run(&mut self)
    {

    }

    fn play_timestep(&mut self)
    {
        self.cars.iter_mut().for_each(|car|{
            let car_pos = self.car_positions.get_mut(&car.id).unwrap();
            if let Some((intersection_id, direction)) = car_pos.current_intersection{
                if !car.at_intersection{ //car is at intersection but not in list, means it must drive 
                    let next_intersection = self.road.get_next_intersection(intersection_id, direction, car.intention.clone()).unwrap();
                    car_pos.in_between = Some( (intersection_id, next_intersection.0, self.road.get_distance(intersection_id, next_intersection.0).unwrap()-1, Direction::get_next_direction(direction, car.intention)));
                    car_pos.current_intersection = None;
                }
            }
            else {
                let in_between = car_pos.in_between.unwrap();
                if in_between.2 == self.road.get_distance(in_between.1, in_between.2).unwrap() -1
                {
                    car_pos.current_intersection = Some((in_between.1, in_between.3));
                    car_pos.in_between = None;
                }
            }
        })
    }



    
}