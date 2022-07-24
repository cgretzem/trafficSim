use crate::traffic_logic::intersection::{TrafficLight, LightStatus};
use std::hash::Hash;

#[derive(Clone, Copy)]
pub enum Direction
{
    Left, 
    Right,
    Straight
}

impl Direction
{
    pub fn get_next_direction(source : u8, intent:Self) -> u8
    {
        match intent{
            Self::Left => (source+1)%4,
            Self::Right => (source-1)%4,
            Self::Straight => (source+2)%4
        }
    }
}

pub struct Car
{
    pub id : u8,
    pub wait_time : usize,
    pub intention : Direction,
    pub at_intersection:bool
}

impl Eq for Car {}

impl PartialEq for Car
{ 
    fn eq(&self, other: &Self) -> bool
    {
        self.id==other.id
    }
}

impl Hash for Car
{
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.id.hash(state);
    }
}

impl Car
{

    pub fn new(id : u8) -> Car
    {
        Car { id, wait_time: 0, intention: Direction::Straight, at_intersection: true }
    }

    pub fn notify(&mut self, main_light_index : usize, lights : &[TrafficLight;4]) -> bool
    {
        
        if self.can_go(lights, main_light_index) == true
        {
            self.at_intersection = false;
            true
        }
        else {
            self.wait_time += 1;
            false
        }
        
    }

    pub fn can_go(&self, lights : &[TrafficLight;4], main_light_index : usize) -> bool
    {
        let forward_light = &lights[main_light_index];
        println!("{:?}", forward_light);
        match self.intention
        {
            Direction::Left =>{
                if let LightStatus::Green = forward_light.left_turn_status {true} else {false}
            },
            
            Direction::Straight | Direction::Right => {
                if let LightStatus::Green = forward_light.main_status {true} else {false} 
            },

        }
    }
    

}