# RustyWidgets

**RustyWidgets** – A modular widget manager for Linux written in Rust. It allows creating and displaying custom widgets using  **HTML/CSS** , with real-time updates and flexible desktop positioning. Inspired by `eww`, but leveraging Rust’s performance and safety.

---

## Features

* Modular widget system – each widget runs independently.
* Widgets written in HTML/CSS (optional JS for limited interactivity).
* Real-time updates and refresh intervals per widget.
* Flexible desktop positioning with transparency and layering.
* Hot-reload support for widget files.
* Configuration via **TOML/JSON** files.

---

## Project Structure

```
RustyWidgets/
│
├─ Cargo.toml
├─ README.md
├─ config/
│   ├─ config.toml
│   └─ widgets/
│       ├─ cpu.toml
│       └─ clock.toml
│
├─ src/
│   ├─ main.rs
│   ├─ core/
│   ├─ renderer/
│   └─ utils/
│
└─ widgets/
    ├─ cpu/
    │   ├─ cpu_widget.html
    │   ├─ cpu_widget.css
    │   └─ cpu_widget.js (optional)
    └─ clock/
        ├─ clock.html
        └─ clock.css

```

---

## Installation

1. **Clone the repository:**

```
git clone https://github.com/yourusername/RustyWidgets.git
cd RustyWidgets

```

1. **Build the project:**

```
cargo build --release
```

3. **Run the application:**

```
cargo run
```
