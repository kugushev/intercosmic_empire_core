use glam::Vec3;
use intercosmic_empire::game::battle::models::route::RouteBuilder;
use intercosmic_empire::game::core::models::stellar_system::spaceport::Spaceport;

#[test]
fn trim_route_tail_check_tail() {
    let mut builder = RouteBuilder::new(Vec3::new(0.0, 0.0, 0.0), Spaceport {
        arrival_radius: 1.0,
        surface_radius: 0.0,
    });

    builder.add_waypoint(Vec3::new(0.0, 0.0, 0.0));
    builder.add_waypoint(Vec3::new(0.9, 0.0, 0.0));

    builder.add_waypoint(Vec3::new(1.0, 0.0, 0.0));

    builder.add_waypoint(Vec3::new(1.1, 0.0, 0.0));
    builder.add_waypoint(Vec3::new(2.0, 0.0, 0.0));

    let route = builder.build(Vec3::new(2.0, 0.0, 0.0), Spaceport {
        arrival_radius: 1.0,
        surface_radius: 0.0,
    });

    assert!(route.is_some());
    assert_eq!(vec![Vec3::new(1.0, 0.0, 0.0), ], route.unwrap().waypoints)
}