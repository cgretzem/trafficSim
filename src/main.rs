use traffic_sim::simulator::{Simulator, Position, Current};
use traffic_sim::traffic_logic::road::Node;
use traffic_sim::traffic_logic::{road::Road};
fn main() {
    //defining road
    let mut road = Road::new();
    road.road.insert(1, [Node::new(1,4,0),
                    Node::new(2, 5, 1),
                    Node::new(3, 3, 2),
                    Node::new(4,1,3)]);
    road.road.insert(2, [Node::new(1,4,2),
                    Node::new(2, 5, 1),
                    Node::new(3, 3, 2),
                    Node::new(4,1,3)]);
    road.road.insert(3, [Node::new(1,4,2),
                    Node::new(1, 5, 3),
                    Node::new(3, 3, 2),
                    Node::new(4,1,3)]);
    road.road.insert(4, [Node::new(1,4,2),
                    Node::new(2, 5, 1),
                    Node::new(3, 3, 2),
                    Node::new(4,1,3)]);
                    
    let mut simulator = Simulator::new(road);

    simulator.add_car(Position::new(Some(Current{int_id:1, direction:0}),None));

    simulator.run(7);

}
