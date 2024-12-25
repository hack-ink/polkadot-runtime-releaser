#!/usr/bin/env bash
set -Eeuo pipefail

DEFAULT_UID=1000
DEFAULT_GID=1000

HOST_UID="${HOST_UID:-}"
HOST_GID="${HOST_GID:-}"

if [ -z "$HOST_UID" ] || [ -z "$HOST_GID" ]; then
  echo "HOST_UID/HOST_GID are not set. Running as root."
  exec "$@"
else
  echo "Running as UID=${HOST_UID}, GID=${HOST_GID}."

  groupadd -g "$HOST_GID" builder
  useradd -m -u "$HOST_UID" -g "$HOST_GID" builder

  exec gosu builder "$@"
fi
