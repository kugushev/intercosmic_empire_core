use glam::Vec3;
use crate::app::game::battle::constants::CONSTANTS;

pub fn generate_waypoints(start: Vec3, anchors: impl Iterator<Item=Vec3>) -> Vec<Vec3> {
    let mut waypoints = vec![];

    let current = start;
    for next in anchors {
        let direction = next - current;
        let mut offset_length = 0.0;
        while offset_length < direction.length() {
            let offset_direction = direction.normalize() * offset_length;
            let offset_position = current + offset_direction;

            let waypoint_direction = (next - offset_position).normalize() * CONSTANTS.gap_between_waypoints;
            let waypoint_position = offset_position + waypoint_direction;

            waypoints.push(waypoint_position);

            offset_length += waypoint_direction.length();

            // check on Null Vector
            if waypoint_direction.length() < CONSTANTS.gap_between_waypoints * 0.01 {
                break;
            }
        }
        waypoints.push(next);
    }

    waypoints
}