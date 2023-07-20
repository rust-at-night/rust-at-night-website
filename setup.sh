#!/bin/bash

SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SCRIPT_DIR="$(dirname "$SCRIPT_PATH")"

cargo install --locked wasm-bindgen-cli trunk just sqlx-cli

# Define the pre-commit hook
read -r -d '' PRE_COMMIT_HOOK <<'EOF'
#!/bin/sh
# Redirect output to stderr.
exec 1>&2
# Enable globbing.
shopt -s globstar

for file in $(git diff --cached --name-only | grep -E '\.sql$')
do
    pg_format --keyword-case 3 $file > formatted.sql
    mv formatted.sql $file
    git add $file
done
EOF

# Get the git directory (this works even in a git submodule)
GIT_DIR=$(git rev-parse --git-dir)

# Create the hooks directory if it doesn't already exist
mkdir -p "$GIT_DIR/hooks"

# Write the hook
echo "$PRE_COMMIT_HOOK" >"$GIT_DIR/hooks/pre-commit"

# Make the hook executable
chmod +x "$GIT_DIR/hooks/pre-commit"

echo "Pre-commit hook for pg_format installed successfully."

rm -rf "$SCRIPT_DIR/website-backend/local" && mkdir -p "$SCRIPT_DIR/website-backend/local" && touch "$SCRIPT_DIR/website-backend/local/local.db"

cd website-backend &&
    sqlx migrate run --source "$SCRIPT_DIR/website-backend/migrations" --database-url "sqlite://$SCRIPT_DIR/website-backend/local/local.db" &&
    cargo sqlx prepare --database-url "sqlite://$SCRIPT_DIR/website-backend/local/local.db"

echo "rust-at-night website project is set up. Happy coding!"
