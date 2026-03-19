//! Emotion enum and controller for the robot face.

/// Six discrete emotions matching typical robotics use cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emotion {
    /// Smile arc down, normal eyes.
    Happy,
    /// Straight mouth, normal eyes.
    Neutral,
    /// Frown arc up, drooping brows.
    Sad,
    /// Flat/angled mouth, angled brows pointing inward and down.
    Angry,
    /// Wide eyes (larger radius), open 'O' mouth.
    Surprised,
    /// Right eye squinting, diagonal mouth to one side.
    Thinking,
}

impl Emotion {
    /// Human-readable label for serial debug output or UI.
    pub fn label(self) -> &'static str {
        match self {
            Emotion::Happy => "Happy",
            Emotion::Neutral => "Neutral",
            Emotion::Sad => "Sad",
            Emotion::Angry => "Angry",
            Emotion::Surprised => "Surprised",
            Emotion::Thinking => "Thinking",
        }
    }

    /// Cycle through all emotions in order (for demo / Tab key).
    pub fn next(self) -> Self {
        match self {
            Emotion::Happy => Emotion::Neutral,
            Emotion::Neutral => Emotion::Sad,
            Emotion::Sad => Emotion::Angry,
            Emotion::Angry => Emotion::Surprised,
            Emotion::Surprised => Emotion::Thinking,
            Emotion::Thinking => Emotion::Happy,
        }
    }
}

/// Owns the current emotion and provides a setter.
pub struct EmotionController {
    pub current: Emotion,
}

impl EmotionController {
    pub fn new() -> Self {
        Self {
            current: Emotion::Neutral,
        }
    }

    pub fn set(&mut self, e: Emotion) {
        self.current = e;
    }
}

impl Default for EmotionController {
    fn default() -> Self {
        Self::new()
    }
}
