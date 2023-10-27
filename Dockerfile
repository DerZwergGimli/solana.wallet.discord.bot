FROM rust:1.70.0 as build

# create a new empty shell project
WORKDIR /walletbot
RUN apt-get update && apt-get install build-essential cmake -y
# copy over your manifests
COPY . .

RUN cd bot && cargo build --release

# our final base
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev openssl ca-certificates
RUN openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.pem -out cert.pem -subj "/C=GE/ST=London/L=London/O=Global Security/OU=IT Department/CN=example.com"


# copy the build artifact from the build stage
COPY --from=build /walletbot/bot/target/release/SolanaDiscordWalletTracker .

# set the startup command to run your binary
CMD ["./SolanaDiscordWalletTracker"]