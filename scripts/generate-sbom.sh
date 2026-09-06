#!/bin/bash
# Frensense SBOM Generator (Bootstrap Version)

VERSION=$(grep '^version =' Cargo.toml | head -n 1 | cut -d '"' -f 2)
NAME=$(grep '^name =' Cargo.toml | head -n 1 | cut -d '"' -f 2)

echo "Generating SBOM for $NAME v$VERSION..."

cat <<EOF > bom.json
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.4",
  "serialNumber": "urn:uuid:$(cat /proc/sys/kernel/random/uuid)",
  "version": 1,
  "metadata": {
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "component": {
      "name": "$NAME",
      "version": "$VERSION",
      "type": "application"
    }
  },
  "components": [
EOF

# Extract dependencies from Cargo.toml
grep "^[a-z]" Cargo.toml -A 50 | grep "=" | grep -vE "^(name|version|edition|license|publish) =" | while read -r line; do
    DEP_NAME=$(echo "$line" | cut -d ' ' -f 1)
    DEP_VER=$(echo "$line" | cut -d '"' -f 2)
    if [[ -n "$DEP_NAME" && -n "$DEP_VER" ]]; then
        echo "    { \"name\": \"$DEP_NAME\", \"version\": \"$DEP_VER\", \"type\": \"library\" }," >> bom.json
    fi
done

# Clean up trailing comma and close JSON
sed -i '$ s/,$//' bom.json
echo "  ]" >> bom.json
echo "}" >> bom.json

echo "[SUCCESS] Generated bom.json"
