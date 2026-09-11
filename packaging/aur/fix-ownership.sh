#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT
#
# Ensure the checkout is owned by the runner user. The container runs as
# root, while builds run as runner via `su runner -c`.
# Idempotent: chown is a fixed-point operation.

set -euo pipefail

chown -R runner:runner .
