#!/usr/bin/env fish

cargo build
alias patanos="$(dirname $(readlink -f $0))"/patanos
funcsave patanos
chmod +x patanos