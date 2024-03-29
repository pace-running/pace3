####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=pace
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /pace

COPY .. .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-slim

RUN apt-get update && apt-get install libpq5 ca-certificates  libssl3 -y
RUN update-ca-certificates

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /pace

# Copy our build
COPY --from=builder /pace/target/release/pace ./pace
COPY --from=builder /pace/templates ./templates

# Use an unprivileged user.
USER pace:pace

CMD ["/pace/pace"]
