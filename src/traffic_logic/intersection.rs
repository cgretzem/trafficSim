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
    light_queues : [Vec<Car>;4]
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
    fn publish(&mut self)//calls notify on all cars so that they may take action
    {
       for i in 0..self.lights.len()
       {
            self.light_queues[i].retain_mut(|car| {
            match car.notify(i+2,&self.lights){
                None => true,
                Some(_dir) => false
            }
        })
       }
            
        
    }
}
