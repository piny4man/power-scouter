
# [Power Scouter](https://power-scouter.shuttleapp.rs/) [![Build](https://github.com/piny4man/power-scouter/actions/workflows/pull-request.yml/badge.svg)](https://github.com/piny4man/power-scouter/actions/workflows/pull-request.yml)
![Scouter](https://github.com/piny4man/power-scouter/blob/main/static/assets/images/scouter.png)

Powerlifting competition points calculator.

Fullstack web application made with 🧡 and Rust.


## Tech Stack

**Client:** [Dioxus](https://dioxuslabs.com/)

**Server:** [Actix Web](https://actix.rs/)

**Deployement** [Shuttle](https://www.shuttle.rs/)

## Roadmap
- [x]  Integer validation on inputs
- [x]  Local points calculation
- [x]  API Score calculation
- [x]  Refactor frontend components
- [x]  Scores hitory
- [x]  Clear inputs after successful score calculation
- [ ]  Add date and competitor data to scores
- [ ]  Improve UI
- [ ]  Optimize frontend build and size


## Website

[Here](https://power-scouter.shuttleapp.rs)


## Run Locally

Clone the project

```bash
  git clone https://github.com/piny4man/power-scouter.git
```

Go to the project directory

```bash
  cd power-scouter
```

(Optinal) Install `cargo make`

```bash
  cargo install cargo-make
```

Start the server

```bash
  cd front
  dioxus serve
```
or with `cargo make` from the root
```bash
  makers serve
```


## Authors

- [@piny4man](https://github.com/piny4man)

