SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
"$SCRIPT_DIR/rebuild.sh" >> /var/log/rebuild.log 2>&1
"$SCRIPT_DIR/run-validator.sh"