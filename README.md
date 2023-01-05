# Cuty.ink Server 🚀🦀

Welcome to the Cuty.ink Server! This project is a Rust server for [Cuty.ink](https://cuty.ink). Built using [Rocket](https://rocket.rs/), [Diesel](https://Diesel.rs), and [SQLite](https://www.sqlite.org/index.html), the server should be able to handle a lot of concurrent requests at the same time. Frontend source code can be found [here](https://github.com/fazatholomew/cuty)

## 🚀 Getting Started

To get started with the Cuty.ink Server, follow these steps:

1. Make sure you have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed on your machine.
2. Clone this repository to your local machine:

```bash
git clone https://github.com/fazatholomew/cuty-server
```

3. Navigate to the cloned repository:

```bash
cd cuty-ink-rocket-rust-server
```

4. Write `.env` file. Below is an example:

```bash
DATABASE_URL=database/database.db
SECRET_KEY=6LdQTcIjAAAAAERkhAFR4Rphe-HqQMAp9hlN_pk2
FRONT_END_URL=http://localhost:5173
```

5. Install Diesel CLI

```bash
cargo install diesel_cli
```

5. Run Diesel migration

```bash
diesel migration run
```

6. Build the project using Cargo:
   `cargo build`
7. Run the server:
   `cargo run`

## 🤝 Contributing

We welcome contributions to the Cuty.ink Server! If you want to contribute, please follow these steps:

1. Fork this repository.
2. Create a new branch for your changes.
3. Make your changes and commit them to your branch.
4. Push your changes to your fork.
5. Open a pull request against the `master` branch of this repository.

Thank you for considering contributing to the Cuty.ink Server! 🎉
