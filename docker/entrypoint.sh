#!/usr/bin/env bash
set -Eeuo pipefail

DEFAULT_UID=1000
DEFAULT_GID=1000

HOST_UID="${HOST_UID:-}"
HOST_GID="${HOST_GID:-}"

if [ -z "$HOST_UID" ] || [ -z "$HOST_GID" ]; then
  echo "HOST_UID/HOST_GID are not set, running as root"
  exec "$@"
else
  echo "running as UID=${HOST_UID}, GID=${HOST_GID}."

  if getent group "${HOST_GID}" >/dev/null 2>&1; then
    EXISTING_GROUP_NAME="$(getent group "${HOST_GID}" | cut -d: -f1)"
    echo "group with GID ${HOST_GID} already exists: ${EXISTING_GROUP_NAME}, reusing that group"
    GROUP_NAME="${EXISTING_GROUP_NAME}"
  else
    GROUP_NAME="builder"
    groupadd -g "${HOST_GID}" "${GROUP_NAME}"
  fi

  if id -u "${HOST_UID}" >/dev/null 2>&1; then
    echo "user with UID ${HOST_UID} already exists, reusing that user"
    USER_NAME="$(id -nu "${HOST_UID}")"
  else
    USER_NAME="builder"
    useradd -m -u "${HOST_UID}" -g "${GROUP_NAME}" "${USER_NAME}"
  fi

  exec gosu "${USER_NAME}" "$@"
fi
