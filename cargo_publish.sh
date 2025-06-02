#!/bin/sh

set -euxo pipefail

(cd src/proc_macro
    cargo publish
)

cargo publish
