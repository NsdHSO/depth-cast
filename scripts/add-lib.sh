#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <crate-name>" >&2
  exit 1
fi

CRATE_NAME="$1"

if [[ ! "$CRATE_NAME" =~ ^[a-zA-Z0-9_-]+$ ]]; then
  echo "Error: crate name must contain only alphanumeric, hyphens, and underscores" >&2
  exit 1
fi

CRATE_DIR="crates/$CRATE_NAME"

if [[ -d "$CRATE_DIR" ]]; then
  echo "Error: $CRATE_DIR already exists" >&2
  exit 1
fi

LIB_NAME="${CRATE_NAME//-/_}"

mkdir -p "$CRATE_DIR/src"

cat > "$CRATE_DIR/Cargo.toml" <<EOF
[package]
name = "$CRATE_NAME"
version = "0.1.0"
edition = "2021"

[lib]
name = "$LIB_NAME"
path = "src/lib.rs"
EOF

cat > "$CRATE_DIR/src/lib.rs" <<EOF
//! $CRATE_NAME — DepthCast library crate.

/// Public entry point for the $CRATE_NAME crate.
pub fn hello() -> &'static str {
    "hello from $CRATE_NAME"
}
EOF

ROOT_CARGO="Cargo.toml"

if ! grep -q "\"crates/$CRATE_NAME\"," "$ROOT_CARGO"; then
  python3 - "$ROOT_CARGO" "$CRATE_NAME" <<'PY'
import sys

path, crate_name = sys.argv[1], sys.argv[2]
member_line = f'    "crates/{crate_name}",'

with open(path, "r") as f:
    content = f.read()

if member_line in content:
    sys.exit(0)

marker = "members = ["
idx = content.index(marker)
insert_at = idx + len(marker)

new_content = content[:insert_at] + "\n" + member_line + content[insert_at:]

with open(path, "w") as f:
    f.write(new_content)
PY
fi

cargo check --workspace

echo "Created $CRATE_DIR and added to workspace members."
