# Rust Pong

A modern, remake of the classic **Pong** arcade game, built using the **Rust** programming language and the **raylib** graphics library. this is my first raylib game, so expect a lot of spaghetti code and a lot of non practical codes.

---

## How to Run

### Prerequisites

1. **Rust Toolchain:** Install via [rustup.rs](https://rustup.rs/)
2. **Dependencies:** Make sure you have the necessary development libraries for your Operating System

### Installation & Launch

Clone the repository and run the project using Cargo:

```bash
git clone https://github.com/streamtechteam/pong-rs.git
cd pong-rs
cargo run --release

```
you may need to also install some other system dependencies

---

## Controls

| Action | Player 1 (Left) | Player 2 (Right) |
| --- | --- | --- |
| **Move Up** | `W` | `Up Arrow` |
| **Move Down** | `S` | `Down Arrow` |
| **Reset Game** | `R` | `R` |
| **Quit** | `Esc` | `Esc` |

---

## Architecture

The game follows a standard **Update-Draw** loop pattern:

1. **Input:** Polls keyboard state for paddle movement.
2. **Update:** Calculates ball trajectories, paddle bounds, and collision detection.
3. **Draw:** Renders the paddles, ball, and score text to the buffer.

---

## Contributing

Contributions are welcome! If you'd like to add features (like AI opponents, sound effects, or a main menu), feel free to:

1. Fork the project.
2. Create your feature branch (`git checkout -b feature/new_feature`).
3. Commit your changes.
4. Open a Pull Request.

---

## License

Distributed under the MIT License. See `LICENSE` for more information.