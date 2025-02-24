use common::payloads::{ RelativeDirection };

pub fn direction_from_angle(angle: f32) -> RelativeDirection {
    if angle >= 315.0 || angle < 45.0 {
        RelativeDirection::Front   // North (0°) ➜ Front
    } else if angle >= 45.0 && angle < 135.0 {
        RelativeDirection::Right   // East (90°) ➜ Right
    } else if angle >= 135.0 && angle < 225.0 {
        RelativeDirection::Back    // South (180°) ➜ Back
    } else {
        RelativeDirection::Left    // West (270°) ➜ Left
    }
}