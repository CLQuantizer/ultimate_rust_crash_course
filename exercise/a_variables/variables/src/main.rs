// ### Part 1
// - Make a new project named `variables` using cargo
// - Open `Cargo.toml`
// - Change the version number to `2.3.4` and save the file.
// - In `src/main.rs` at the start of the `main()` function:

// - Declare the variable `missiles` and initialize it to `8`
// - Declare the variable `ready` and initialize it to `2`
// - Change the `println!(...)` at the end of `main()` to:
// - `println!("Firing {} of my {} missiles...", ready, missiles);`
// - [ ] Run your program using cargo (see "cargo help" if you forgot the command).
//   Some common errors you may hit:
//   - Forgot to use `let` to bind a variable
//   - Forgot a semicolon `;` at the end of a line

fn main() {
    let missiles=8;
    let ready=2;
    println!("Firing {} of my {} missiles...", ready, missiles);
}
