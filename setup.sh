#!/bin/bash

SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SCRIPT_DIR="$(dirname "$SCRIPT_PATH")"

# Install deps
cargo install --locked wasm-bindgen-cli trunk just sqlx-cli sleek

# Define the pre-commit hook for enforced SQL formatting
read -r -d '' PRE_COMMIT_HOOK <<'EOF'
#!/bin/bash
sleek --indent-spaces 4 --uppercase ./website-backend/migrations/**/*.sql
EOF

# Get the git directory (this works even in a git submodule)
GIT_DIR=$(git rev-parse --git-dir)

# Create the hooks directory if it doesn't already exist
mkdir -p "$GIT_DIR/hooks"

# Write the hook
echo "$PRE_COMMIT_HOOK" >"$GIT_DIR/hooks/pre-commit"

# Make the hook executable
chmod +x "$GIT_DIR/hooks/pre-commit"

echo "Pre-commit hook installed successfully."

rm -rf "$SCRIPT_DIR/website-backend/local" && mkdir -p "$SCRIPT_DIR/website-backend/local" && touch "$SCRIPT_DIR/website-backend/local/local.db"

cd website-backend &&
    sqlx migrate run --source "$SCRIPT_DIR/website-backend/migrations" --database-url "sqlite://$SCRIPT_DIR/website-backend/local/local.db" &&
    cargo sqlx prepare --database-url "sqlite://$SCRIPT_DIR/website-backend/local/local.db"

echo "rust-at-night website project is set up. Happy coding!"
