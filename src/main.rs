// If the target architecture is 16-bit (like the Game Boy)
#[cfg(target_pointer_width = "16")]
pub type Int = u8;

// If the target architecture is 64-bit (like your Linux machine)
#[cfg(target_pointer_width = "64")]
pub type Int = i32;

fn main() {
  println!("Hello, world!");
}
