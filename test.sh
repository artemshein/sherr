#!/usr/bin/env bash

cargo test --features impl -- --nocapture --test-threads=1
