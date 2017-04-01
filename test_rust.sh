#!/usr/bin/env bash
set -e

function cleanup() {
    kill "$(<./rust/rust_lms.pid)" && rm -f ./rust/rust_lms.pid
}

cd "$(dirname "$0")"

(cd rust
 cargo build
 ./target/debug/rust_lms&
 echo $! > rust_lms.pid)

trap cleanup EXIT

(cd local-matching-service-tests
mvn test -DMATCHING_URL=http://localhost:3003/rust/matching-service -DUSER_ACCOUNT_CREATION_URL=http://localhost:3003/rust/account-creation)

