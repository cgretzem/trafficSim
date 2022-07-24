#[allow(dead_code)]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{hash::Hash, fmt::Display};

use super::car::Direction;




#[derive(Clone, Copy, PartialEq, Eq,Debug)]
pub enum LightStatus
{
    Green,
    Yellow,
    Red
}

impl Display for LightStatus{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        match self{
            Self::Green => output.push_str("Green"),
            Self::Yellow => output.push_str("Yellow"),
            Self::Red => output.push_str("Red"),
        }
        write!(f,"{}", output)
    }
}

impl Distribution<LightStatus> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LightStatus {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=2) { // rand 0.8
            0 => LightStatus::Green,
            1 => LightStatus::Yellow,
            _ => LightStatus::Red,
        }
    }
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TrafficLight
{   
    pub main_status : LightStatus,
    pub left_turn_status: LightStatus
}

impl TrafficLight{
    pub fn rand() -> TrafficLight{
        TrafficLight { main_status: rand::random(), left_turn_status: rand::random() }
    }
}

#[derive(Debug)]
pub struct Intersection
{
    pub id : u8,
    pub lights: [TrafficLight; 4],
    pub light_queues : [Vec<u8>;4]
}

impl Eq for Intersection {}

impl PartialEq for Intersection
{ 
    fn eq(&self, other: &Self) -> bool
    {
        self.id==other.id
    }
}

impl Hash for Intersection
{
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.id.hash(state);
    }
}

impl Intersection
{
    pub fn new(id: u8) -> Intersection{
        Intersection{id, lights: [TrafficLight::rand();4], light_queues:[Vec::new(), Vec::new(), Vec::new(), Vec::new()]}
    }

    pub fn add_car_to_queue(&mut self, car_id:u8, dir:u8){
        self.light_queues[usize::from(dir)].push(car_id);
    }

    pub fn get_lights(&self, direction : u8) -> TrafficLight{
        self.lights[usize::from(direction)]
    } 
}
