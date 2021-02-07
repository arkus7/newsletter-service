#!/usr/bin/env bash
set -eo pipefail

SCRIPTS_DIR=$(dirname "$0")
. "${SCRIPTS_DIR}/wait_for_db.sh"

sqlx database create