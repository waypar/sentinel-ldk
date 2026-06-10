#base image
FROM rust:1-trixie

WORKDIR /workspace

COPY vendor/linux/ /workspace/vendor/linux

RUN set -eux; \
    ARCH=$(dpkg --print-architecture); \
    echo "Detected architecture: $ARCH"; \
    if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then \
        DEB_ARCH="arm64"; \
        HASPLMD_ARCH="arm64"; \
    elif [ "$ARCH" = "amd64" ] || [ "$ARCH" = "x86_64" ]; then \
        DEB_ARCH="amd64"; \
        HASPLMD_ARCH="x86_64"; \
    else \
        echo "Unsupported architecture: $ARCH"; \
        exit 1; \
    fi; \
    dpkg -x vendor/linux/Sentinel-LDK/Redistribute/Runtime/aksusbd_*_${DEB_ARCH}.deb / && \
    groupadd lms_group && \
    useradd -g lms_group lms_user && \
    chown -R lms_user:lms_group /var/hasplm && \
    chown -R lms_user:lms_group /etc/hasplm && \
    chmod +x /usr/sbin/hasplmd_* && \
    echo "$HASPLMD_ARCH" > /etc/hasplmd_arch

RUN apt-get update && apt-get install -y libclang-dev
RUN rustup component add rustfmt

USER lms_user

# Load architecture and run the correct hasplmd
ENTRYPOINT ["/bin/sh", "-c", "HASPLMD_ARCH=$(cat /etc/hasplmd_arch) && exec /usr/sbin/hasplmd_${HASPLMD_ARCH} -f"]
