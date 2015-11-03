struct PlayerInputSettings {
    movement_x: Vec<String>,
    movement_y: Vec<String>,
    fire: String,
}

struct InputSettings {
    players: Vec<PlayerInputSettings>,
}

struct Settings {
    full_screen: bool,
    screen_size: (usize,usize),
    input: InputSettings,
}
