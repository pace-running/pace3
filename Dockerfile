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

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

RUN apt-get update && apt-get install libpq5 -y

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /pace

# Copy our build
COPY --from=builder /pace/target/release/main ./pace
COPY --from=builder /pace/static ./static
COPY --from=builder /pace/templates ./templates

# Try to add frontend

WORKDIR /frontend/pace-ui

RUN apt-get update && apt-get install nodejs npm -y

RUN npm run build
RUN npm run start

# Use an unprivileged user.
USER pace:pace

CMD ["/pace/pace"]
