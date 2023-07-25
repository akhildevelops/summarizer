# summarizer
Podcast and Youtube

# Setup
- Install rust from https://www.rust-lang.org/tools/install if not present in the system.
- Clone the repo
- Install sqlx-cli - `cargo install sqlx-cli --no-default-features --features native-tls,postgres`
- Point existing postgresql (or install) through environment variable in the terminal - `export DATABASE_URL=postgresql://<username>:<password>@<hostname>:<port>/<database>`
- Get OpenAI Key and export 

## Cross-compilation to raspberrypi
- Install podman or docker
- Install cross cli - `cargo install cross --git https://github.com/cross-rs/cross`
- Run `cross build --features=vendored-ssl --target=arm-unknown-linux-gnueabihf`

