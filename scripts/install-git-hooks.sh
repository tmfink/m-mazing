#!/bin/sh

set -eux

cd "$(dirname -- "$0")"/..

ln -sfr "scripts/check_all.sh" ".git/hooks/pre-commit"