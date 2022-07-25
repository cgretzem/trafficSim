use rand::{Rng, prelude::Distribution, distributions::Standard};

use crate::traffic_logic::intersection::{TrafficLight, LightStatus};
use std::hash::Hash;

#[derive(Clone, Copy)]
///Enum to represent directions a car can drive at an intersection
pub enum Direction
{
    Left, 
    Right,
    Straight,
    UTurn
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=3) { // rand 0.8
            0 => Direction::Left,
            1 => Direction::Right,
            2=> Direction::UTurn,
            _ => Direction::Straight,
        }
    }
}


impl Direction
{
    ///gets the next cardinal direction for the car given a source direction and the driver's intent
    /// # Parameters
    /// - `source` : `u8` - the cardinal direction in which the car is sitting relative to the intersection
    /// - `intent` : `Direction` - The direction that the driver wants to turn
    /// # Returns
    /// `next_direction` - `u8` - The next cardinal direction based on the turn
    /// 
    ///# Examples
    /// 
    /// ```rust
    /// use traffic_sim::traffic_logic::car::Direction;
    /// let source_dir = 3; // North :0, East:1.. 
    /// let intent = Direction::Left;
    /// //turning left from a car sitting at the 
    /// //west end of the intersection should result
    /// //in the car going north
    /// assert_eq!(Direction::get_next_direction(source_dir, intent), 0);
    /// assert_eq!(Direction::get_next_direction(source_dir, Direction::Right), 2);
    /// assert_eq!(Direction::get_next_direction(source_dir, Direction::Straight), 1);
    /// ```
    pub fn get_next_direction(source : u8, intent:Self) -> u8
    {
        match intent{
            Self::Left => (source+1)%4,
            Self::Right => (source+3)%4,
            Self::Straight => (source+2)%4,
            _ => source
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

    pub fn randomize_intent(&mut self){
        self.intention = rand::random();
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
        //println!("{:?}", forward_light);
        match self.intention
        {
            Direction::Left =>{
                if let LightStatus::Green = forward_light.left_turn_status {true} else {false}
            },
            
            Direction::Straight | Direction::Right => {
                if let LightStatus::Green = forward_light.main_status {true} else {false} 
            },

            _ => {
                if let LightStatus::Green = forward_light.left_turn_status {true} else {false}
            }

        }
    }
    

}