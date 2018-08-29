#!/usr/bin/env bash

cargo test --features impl -- --nocapture && \
cargo test --features fail -- --nocapture && \
cargo test --features impl,fail -- --nocapture
