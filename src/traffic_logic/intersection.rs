use crate::traffic_logic::car::Car;
use std::hash::Hash;




#[derive(Clone, Copy)]
pub enum LightStatus
{
    Green,
    Yellow,
    Red
}


#[derive(Clone, Copy)]
pub struct TrafficLight
{   
    pub main_status : LightStatus,
    pub left_turn_status: LightStatus
}

pub struct Intersection
{
    id : u8,
    lights: [TrafficLight; 4],
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
    
}
