#!/usr/bin/env bash

source "approvals.bash"

approve "cargo run" && git add --all;git commit -m wip