mod emulator;
fn main() {
    let mut emulator = emulator::emulator::Emulator::new();
    emulator.load_file_from_path("demos/blink.bin");
    emulator.run();
}
