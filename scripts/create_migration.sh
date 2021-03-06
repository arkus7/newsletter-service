#!/usr/bin/env bash
set -eo pipefail

MIGRATION_NAME=$1

[[ -z "${MIGRATION_NAME}" ]] && {
  echo "Missing migration name."
  echo "Usage: ./create_migration.sh <migration_name>"
  exit 1
}

SCRIPTS_DIR=$(dirname "$0")

. "${SCRIPTS_DIR}/wait_for_db.sh"

echo "Creating migration ${MIGRATION_NAME}..."
sqlx migrate add "${MIGRATION_NAME}"
echo "Migration ${MIGRATION_NAME} created successfully!"