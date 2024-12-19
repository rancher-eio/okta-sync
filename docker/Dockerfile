ARG SOURCE_BUILD_IMAGE
ARG SOURCE_BUILD_TAG
ARG SOURCE_GIT_IMAGE
ARG SOURCE_GIT_TAG
ARG SOURCE_RUNTIME_IMAGE
ARG SOURCE_RUNTIME_TAG
ARG SOURCE_UPSTREAM_REPOSITORY
ARG SOURCE_UPSTREAM_TAG

FROM --platform=${TARGETARCH} ${SOURCE_GIT_IMAGE}:${SOURCE_GIT_TAG} AS git
WORKDIR /src

FROM --platform=${TARGETARCH} ${SOURCE_BUILD_IMAGE}:${SOURCE_BUILD_TAG} AS builder
WORKDIR /src

FROM git AS checkout
ARG SOURCE_UPSTREAM_REPOSITORY
ARG SOURCE_UPSTREAM_TAG
RUN git clone --single-branch --branch="${SOURCE_UPSTREAM_TAG}" "https://${SOURCE_UPSTREAM_REPOSITORY}" .

FROM scratch AS sources
COPY --from=checkout /src /

FROM builder as fetch
ENV CARGO_HOME="/srv/cargo"
COPY --from=checkout /src/ /src/
RUN cargo fetch --locked

FROM scratch AS dependencies
COPY --from=fetch /srv/cargo /

FROM builder AS build
ENV CARGO_HOME="/srv/cargo"
ENV CARGO_INSTALL_ROOT="/usr/local"
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS="1"
ENV CARGO_PROFILE_RELEASE_DEBUG="false"
ENV CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS="false"
ENV CARGO_PROFILE_RELEASE_INCREMENTAL="false"
ENV CARGO_PROFILE_RELEASE_LTO="true"
ENV CARGO_PROFILE_RELEASE_OPT_LEVEL="s"
ENV CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS="false"
ENV CARGO_PROFILE_RELEASE_PANIC="unwind"
ENV CARGO_PROFILE_RELEASE_RPATH="false"
COPY --from=sources / ./
COPY --from=dependencies / /srv/cargo/
RUN cargo build --workspace --profile=release

FROM scratch AS artifacts
COPY --from=build /src/target/release/eio-okta-client /usr/local/bin/
COPY --from=build /src/target/release/eio-okta-sync /usr/local/bin/

FROM --platform=${TARGETARCH} ${SOURCE_RUNTIME_IMAGE}:${SOURCE_RUNTIME_TAG} AS runtime
COPY --from=sources /LICENSE-MIT /
COPY --from=sources /LICENSE-APACHE /
COPY --from=artifacts /usr/local/bin/ /usr/local/bin/
RUN ln -s /usr/local/bin/{eio-,}okta-client
RUN ln -s /usr/local/bin/{eio-,}okta-sync
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
ENTRYPOINT [ "/usr/local/bin/okta-sync" ]
