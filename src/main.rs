use traffic_sim::simulator::{Simulator, Position, Current};

fn main() {
    //defining road
    // road.road.insert(1, [Node::new(1,4,0),
    //                 Node::new(2, 5, 1),
    //                 Node::new(3, 3, 2),
    //                 Node::new(4,1,3)]);
    // road.road.insert(2, [Node::new(1,4,2),
    //                 Node::new(2, 5, 1),
    //                 Node::new(3, 3, 2),
    //                 Node::new(4,1,3)]);
    // road.road.insert(3, [Node::new(1,4,2),
    //                 Node::new(1, 5, 3),
    //                 Node::new(3, 3, 2),
    //                 Node::new(4,1,3)]);
    // road.road.insert(4, [Node::new(1,4,2),
    //                 Node::new(2, 5, 1),
    //                 Node::new(3, 3, 2),
    //                 Node::new(4,1,3)]);
                    
    let mut simulator = Simulator::new();
    simulator.add_intersections(4);


    simulator.add_road(1, 1, 2, 5);
    simulator.add_road(1, 2, 3, 3);
    simulator.add_road(1, 3, 4, 6);
    simulator.add_car(Position::new(Some(Current{int_id:1, direction:1}),None));

    simulator.run(10);

}
