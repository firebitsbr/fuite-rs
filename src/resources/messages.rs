use amethyst::{
    core::nalgebra::Vector3,
    shrev::EventChannel
};


/// Messages sent between systems.
#[derive(Debug, PartialEq)]
pub enum Message {
    // Camera
    CameraMoveX(f32),
    CameraMoveY(f32),

    // Game
    LevelStarted,
    DeadGoodBot,
    DeadBadBot,

    // Player input
    MouseLeftClick(Vector3<f32>)
}

/// Channel where messages are sent.
pub type MessageChannel = EventChannel<Message>;
