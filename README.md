# Summarizer
Podcast and Youtube

# Setup
- Install rust from https://www.rust-lang.org/tools/install if not present in the system.
- Clone the repo
- Install sqlx-cli - `cargo install sqlx-cli --no-default-features --features native-tls,postgres`
- Point existing postgresql (or install) through environment variable in the terminal - `export DATABASE_URL=postgresql://<username>:<password>@<hostname>:<port>/<database>`
- Get OpenAI Key and export as a env variable: `export OPENAI_API_KEY=**********`

# Start the service
### Hub
- This is the hub that accepts jobs - `cargo run --bin controller`
### Client
- Open a new terminal.
- This will submit jobs to hub - `sqlx migrate run --ignore-missing && cargo run --bin jobs`
### Server
- A restapi to interact with the service - `cargo run --bin server`
### UI
- A small piece of UI to see all the summaries - `cd ui && npm install && VITE_SUMMARIZER_URL=http://localhost:3001/api/v1 npm run dev`

## Cross-compilation to raspberrypi
- Install podman or docker
- Install cross cli - `cargo install cross --git https://github.com/cross-rs/cross`
- Run `cross build --release --features=vendored-ssl --target=arm-unknown-linux-gnueabihf`
- Find binaries in `target/arm-unknown-linux-gnueabihf/release/`
- Port them to raspberry pi device and follow above steps of starting of service

