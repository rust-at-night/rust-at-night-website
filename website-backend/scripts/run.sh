#!/bin/bash
set -e

litesteam --version
# These need to be set in the Dockerfile.
# If testing locally local.
echo "LS_DB_PATH: $LS_DB_PATH"
echo "LS_REPLICA_URL: $LS_REPLICA_URL"

# Restore the database if it does not already exist.
if [ -f "$LS_DB_PATH" ]; then
    echo "Database already exists, skipping restore"
else
    # This is a production deployment scenario where we would never copy the database from local.
    # Instead we'd first check if there is a replica.
    echo "No database found, restoring from replica if exists"
    RESTORE_OUTPUT=$(litestream restore -v -if-replica-exists -o "${LS_DB_PATH}" "${LS_REPLICA_URL}")

    # -if-replica-exists
    # Returns exit code of 0 if backups found.
    # https://litestream.io/reference/restore/
    # But this is not true.. unfortunately it always exists with 1.
    # We can not use the code here.
    # EXIT_CODE=$?
    # echo "Exit code: $EXIT_CODE"

    # # If there is no replica we'll just create an empty database.
    # if [ $EXIT_CODE -eq 0 ]; then
    #     echo "No backups found. Instead of restoring, creating an empty database."
    #     mkdir -p /data && touch /data/db
    # fi

    # TODO:
    # Instead there is a hack but it is quite dependent to the command output and should be replaced.
    if [[ "$RESTORE_OUTPUT" == *"no matching backups"* ]]; then
        echo "No backups found. Instead of restoring, creating an empty database."
        mkdir -p /data && touch /data/main.db
    fi

fi

# Run litestream with your app as the subprocess.
exec litestream replicate -exec "/usr/local/bin/website_backend -dsn ${LS_DB_PATH}"
