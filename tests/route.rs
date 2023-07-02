use glam::Vec3;
use intercosmic_empire::old::battle::models::route::RouteBuilder;
use intercosmic_empire::old::core::models::stellar_system::spaceport::Spaceport;

#[test]
fn trim_route_tail_check_tail() {
    let id = 42;
    let mut builder = RouteBuilder::new(
        id,
        Vec3::new(0.0, 0.0, 0.0),
        Spaceport {
            orbit_radius: 1.0,
            surface_radius: 0.0,
        });

    builder.add_waypoint(Vec3::new(0.0, 0.0, 0.0), id);
    builder.add_waypoint(Vec3::new(0.9, 0.0, 0.0), id);

    builder.add_waypoint(Vec3::new(1.0, 0.0, 0.0), id);

    builder.add_waypoint(Vec3::new(1.1, 0.0, 0.0), id);
    builder.add_waypoint(Vec3::new(2.0, 0.0, 0.0), id);

    let route = builder.build(&Vec3::new(2.0, 0.0, 0.0), &Spaceport {
        orbit_radius: 1.0,
        surface_radius: 0.0,
    },
    id
    );

    assert!(route.is_some());
    assert_eq!(vec![Vec3::new(1.0, 0.0, 0.0)], route.unwrap().waypoints)
}