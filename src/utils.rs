/// Clears the terminal screen for both Windows and Unix systems.
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}