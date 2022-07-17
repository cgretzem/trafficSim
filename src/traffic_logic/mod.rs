#[derive(Clone, Copy)]
pub enum LightStatus
{
    Green,
    Yellow,
    Red
}
#[derive(Clone, Copy)]
pub enum Direction
{
    Left, 
    Right,
    Straight
}
#[derive(Clone, Copy)]
pub struct TrafficLight
{   
    main_status : LightStatus,
    left_turn_status: LightStatus
}

pub struct Road
{
    //cars are on roads, road keeps track of what happens when a car goes a certain direction at a certain intersection
    //graph structure, nodes are intersections
    //simulator keeps track of car position on road, when car is not in intersections waiting list and is positioned at intersection,
    //simulator calls follow_road(car, intersection) and uses car's intention to palce car at next intersection
    road : Vec<(Intersection, [(Intersection, u8);3])> //graph linking each intersection with a list of its destination intersections and the number of ticks it takes to get there

}


pub struct Car
{
    wait_time : usize,
    intention : Direction,
    at_intersection:bool
}

pub struct Intersection
{
    lights: [TrafficLight; 4],
    light_queues : [Vec<Car>;4]
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

impl Car
{
    pub fn notify(&mut self, main_light_index : usize, lights : &[TrafficLight;4]) -> Option<Direction>
    {
        
        if self.can_go(lights, main_light_index) == true
        {
            self.at_intersection = false;
            Some(self.intention)
        }
        else {
            None
        }
        
    }

    fn can_go(&self, lights : &[TrafficLight;4], main_light_index : usize) -> bool
    {
        let forward_light = &lights[main_light_index];
        let left_light = &lights[main_light_index-1];
        let right_light = &lights[main_light_index +1];

        match self.intention
        {
            Direction::Left =>{
                if let LightStatus::Green = left_light.left_turn_status {true} else {false}
            },
            
            Direction::Straight | Direction::Right => {
                if let LightStatus::Green = forward_light.main_status {true} else {false} 
            },

        }
    }
}

