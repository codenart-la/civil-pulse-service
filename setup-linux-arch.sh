#!/bin/bash
pacman -Sy just curl wget
./scripts/setup-posix-common.sh
source "$HOME/.cargo/env"