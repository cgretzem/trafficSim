
use crate::traffic_logic::{road::Road, car::{Car, Direction}, intersection::{Intersection, TrafficLight}};
use std::{collections::HashMap};
#[allow(dead_code)]
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

#[derive(Clone, Copy)]
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
    pub current_intersection: Option<Current>,

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
    pub fn new(current_intersection : Option<Current>, in_between : Option<Between>) -> Position
    {
        Position { current_intersection, in_between}
    }
    

}

pub struct Simulator
{
    road:Road,
    car_positions: HashMap<u8, Position>,
    cars : Vec<Car>,
    intersections : Vec<Intersection>,
    timestep: usize,
    next_int_id : u8,
    next_car_id : u8

}

impl Simulator
{
    pub fn new(road:Road) -> Simulator
    {
        let mut intersections:Vec<Intersection> = Vec::new();
        road.road.iter().for_each(|(int_id, lights)|{
            let new_intersection = Intersection::new(*int_id);
            intersections.push(new_intersection);
        });
        Simulator{road, car_positions: HashMap::new(), timestep:0, cars:Vec::new(), intersections, next_car_id: 0, next_int_id:0}
    }


    pub fn run(&mut self, ticks:usize)
    {
        for _ in 0..ticks{
            self.play_timestep();
            println!("\nTimestep {}\n------------------------", self.timestep);
            self.car_positions.iter().for_each(|(car_id,pos)|{
                println!("Car : {}",car_id);
                match pos.current_intersection{

                    None => {
                        let in_between = pos.in_between.unwrap();
                        println!("Is {}/{} remaining between Intersection {} and Intersection {}",
                        in_between.distance_to_target,
                        Road::get_distance(&self.road, in_between.int_1_id, in_between.int_2_id).unwrap(),
                        in_between.int_1_id, in_between.int_2_id);
                    },
                    Some(current) => {
                        println!("Is at intersection {} waiting at direction {}", current.int_id, current.direction);
                        let intersec = self.get_intersection(current.int_id).unwrap_or_else(|| panic!("No intersection found with ID {}", current.int_id)).lights;
                        println!("Intersection lights for car waiting in direction {}: \nLeft Turn: {}\nMain: {}", current.direction, intersec[(usize::from(current.direction+2))%4].left_turn_status, intersec[(usize::from(current.direction+2))%4].left_turn_status)
                    }
                };
            });
            self.timestep +=1;
        }
        
        
    }

    pub fn add_car(&mut self, pos:Position){
        self.cars.push(Car::new(self.next_car_id));
        let car_id = self.next_car_id;
        let int_id = pos.current_intersection.expect("Car does not have a current intersection").int_id;
        let intersection = self.get_intersection_mut(int_id).unwrap_or_else(|| panic!("Could not find intersection with ID {}", int_id));
        intersection.add_car_to_queue(car_id, pos.current_intersection.unwrap().direction);
        self.car_positions.insert(self.next_car_id, pos);
        self.next_car_id += 1;
    }

    fn create_random_lights(&self) ->  HashMap<u8, [TrafficLight;4]>
    {
        let mut new_map: HashMap<u8, [TrafficLight;4]> = HashMap::new();
        self.intersections.iter().for_each(|intersection| {
            let id = intersection.id.clone();
            let random_lights: [TrafficLight;4] = [TrafficLight::rand(), TrafficLight::rand(), TrafficLight::rand(), TrafficLight::rand()];
            new_map.insert(id, random_lights);
        });
        new_map
    }


    fn tick_cars(&mut self)
    {
        self.cars.iter_mut().for_each(|car|{
            let car_pos = self.car_positions.get_mut(&car.id).unwrap();
            if let Some(current) = car_pos.current_intersection{
                let intersection_id = current.int_id;
                let direction = current.direction;
                if !car.at_intersection{ //car is at intersection but not in list, means it must drive 
                    let next_intersection = self.road.get_next_intersection(intersection_id, direction, car.intention.clone()).unwrap();
                    if next_intersection.0 != 0{ // if car is not at dead end
                       
                    
                    let new_in_between = Between{
                                                    int_1_id: intersection_id,
                                                    int_2_id: next_intersection.0,
                                                    distance_to_target: self.road.get_distance(intersection_id, next_intersection.0).unwrap()-1,
                                                    from: Direction::get_next_direction(direction, car.intention)};
                    car_pos.in_between = Some( new_in_between );
                    car_pos.current_intersection = None;
                    car.at_intersection = false;
                    }
                }
            }
            else {
                let mut in_between = car_pos.in_between.unwrap();
                if in_between.distance_to_target == 1
                {
                    let intersection = self.intersections.iter_mut()
                    .find(|int| int.id == in_between.int_2_id).unwrap();
                    if car.can_go(&intersection.lights, usize::from(in_between.from+2)%4) {//lights at target intersection are green
                        let next_intersection = self.road.get_next_intersection(intersection.id, in_between.from+2, car.intention.clone()).unwrap();
                        if next_intersection.0 != 0{ // if car is not at dead end
                        
                        
                        let new_in_between = Between{
                                                        int_1_id: intersection.id,
                                                        int_2_id: next_intersection.0,
                                                        distance_to_target: self.road.get_distance(intersection.id, next_intersection.0).unwrap(),
                                                        from: Direction::get_next_direction(in_between.from, car.intention)};
                        car_pos.in_between = Some( new_in_between );
                        car_pos.current_intersection = None;
                        car.at_intersection = false;
                        }
                        return;
                    }


                    let new_curr = Current{int_id: in_between.int_2_id, direction: in_between.from};
                    car_pos.current_intersection = Some(new_curr);
                    self.intersections.iter_mut()
                    .find(|int| int.id == in_between.int_2_id).unwrap()
                    .add_car_to_queue(car.id, new_curr.direction);
                    
                    car_pos.in_between = None;
                    car.at_intersection = true;
                    
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
            let mut i:u8 = 0;
            let intersection = self.intersections.iter_mut().find(|int_id|{
                int_id.id == *id
            }).unwrap();
            //println!("{:?}", intersection);
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
            let changed = self.get_car_mut(*car_id).unwrap().notify(usize::from(*main_light_index), new_lights.get(intersection_id).unwrap());
            self.get_intersection_mut(*intersection_id).unwrap().light_queues.iter_mut().for_each(|vec|{
                vec.retain(|c_id|{
                    if changed{
                        *c_id != *car_id
                    }
                    else{
                        true
                    }
                    
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

    fn get_intersection(&self, id:u8) -> Option<&Intersection>{
        self.intersections.iter().find(|int| int.id == id)
    }

    fn get_pos_mut(&mut self, car_id:u8)-> Option<&mut Position>
    {
        self.car_positions.get_mut(&car_id)
    }

    fn play_timestep(&mut self)
    {
        self.tick_lights(self.create_random_lights());
        self.tick_cars();
    }



    
}