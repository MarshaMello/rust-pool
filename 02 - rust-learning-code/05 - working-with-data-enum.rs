enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn ways(go : Direction) {
    match go {
        Direction :: Up => "up",
        Direction :: Down => "down",
        Direction :: Left => "left",
        Direction :: Right => "right"
    }
}