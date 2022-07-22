use crate::traffic_logic::{road::Road, car::{Car, Direction}, intersection::{Intersection, TrafficLight, self}};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy)]
pub struct Between
{
    //The IntersectionID of the intersection the car just left
    pub int_1_id:u8,
    ///The IntersectionID of the intersection the car is going to
    pub int_2_id:u8,
    //How far the car is from the target intersection
    pub distance_to_target:u8,
    //The direction the car is coming from/will arrive at
    pub from:u8,
}

pub struct Current{
    //The IntersectionID of the intersection
    pub int_id : u8,
    ///The direction the car is sitting at
    pub direction: u8
}


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
    pub in_between : Option<Between>
}

impl Position
{
    fn new(current_intersection : Option<(u8, u8)>, in_between : Option<Between>) -> Position
    {
        Position { current_intersection, in_between}
    }
    
    fn get_distance(&self) -> u8
    {
        if let None = self.in_between
        {
            panic!("Cannot call get distance when car is not between 2 intersections");
        }
        self.in_between.as_ref().unwrap().distance_to_target
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


    fn tick_cars(&mut self)
    {
        self.cars.iter_mut().for_each(|car|{
            let car_pos = self.car_positions.get_mut(&car.id).unwrap();
            if let Some((intersection_id, direction)) = car_pos.current_intersection{
                if !car.at_intersection{ //car is at intersection but not in list, means it must drive 
                    let next_intersection = self.road.get_next_intersection(intersection_id, direction, car.intention.clone()).unwrap();
                    let new_in_between = Between{
                                                    int_1_id: intersection_id,
                                                    int_2_id: next_intersection.0,
                                                    distance_to_target: self.road.get_distance(intersection_id, next_intersection.0).unwrap()-1,
                                                    from: Direction::get_next_direction(direction, car.intention)};
                    car_pos.in_between = Some( new_in_between );
                    car_pos.current_intersection = None;
                } 
            }
            else {
                let mut in_between = car_pos.in_between.unwrap();
                if in_between.distance_to_target == 1
                {
                    car_pos.current_intersection = Some((in_between.int_2_id, in_between.from));
                    self.intersections.get_mut(usize::from(in_between.int_2_id)).unwrap().light_queues[usize::from((in_between.from+2)%4)].push(car.id); // subscribing car to light
                    
                    car_pos.in_between = None;

                }
                else {
                    in_between.distance_to_target -= 1;
                    car_pos.in_between = Some(in_between);
                }
            }
        })
    }
    

    fn tick_lights(&mut self, new_lights: HashMap<u8, [TrafficLight;4]>)
    {
        let mut ids_to_notify: Vec<(u8, u8, u8)> = Vec::new(); // vector holds car IDs followed by main light index, then intersection ID
        for (id, new) in new_lights.iter()
        {
            let cars_to_update: Vec<u8> = Vec::new();
            let mut i:u8 = 0;
            let intersection = self.intersections.iter_mut().find(|int_id|{
                int_id.id == *id
            }).unwrap();
            intersection.light_queues.iter_mut()
            .for_each(|q| {
                q.iter().for_each(|x| {
                    ids_to_notify.push((*x, (i+2)%4, intersection.id))
                });
                    i+=1;
            });
            intersection.lights = *new;

        }
        //notify the cars and remove from list
        ids_to_notify.iter().for_each(|(car_id, main_light_index, intersection_id)|{
            self.get_car_mut(*car_id).unwrap().notify(usize::from(*main_light_index), new_lights.get(intersection_id).unwrap());
            self.get_intersection_mut(*intersection_id).unwrap().light_queues.iter_mut().for_each(|vec|{
                vec.retain(|c_id|{
                    *c_id == *car_id
                })
            });
        })


    }

    fn get_car_mut(&mut self, id:u8) -> Option<&mut Car>
    {
        self.cars.iter_mut().find(|car| car.id == id)
    }

    fn get_car(&self, id:u8) -> Option<&Car>{
        self.cars.iter().find(|car| car.id==id)
    }

    fn get_intersection_mut(&mut self, id:u8) -> Option<&mut Intersection>
    {
        self.intersections.iter_mut().find(|int| int.id == id)
    }

    fn get_pos_mut(&mut self, car_id:u8)-> Option<&mut Position>
    {
        self.car_positions.get_mut(&car_id)
    }

    fn play_timestep(&mut self)
    {
       self.tick_cars();
    }



    
}