use traffic_sim::simulator::{Simulator, Position, Current};

fn main() {

    let mut simulator = Simulator::new();
    simulator.add_intersections(4);


    simulator.add_road(1, 1, 2, 5);//East
    simulator.add_road(1, 2, 3, 3);//South
    simulator.add_road(1, 3, 4, 6);//West

    simulator.add_car(Position::new(Some(Current{int_id:1, direction:1}),None));
    simulator.add_car(Position::new(Some(Current{int_id:1, direction:1}),None));

    simulator.run(10);

}
