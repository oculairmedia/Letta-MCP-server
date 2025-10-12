#!/bin/bash

# Find all JS files in src/tools
find src/tools -name "*.js" | while read file; do
    # Check if file contains object schemas
    if grep -q "type: 'object'" "$file"; then
        echo "=== Checking $file ==="
        # Use awk to find object schemas without additionalProperties
        awk '
        /type: .object./ {
            obj_line = NR
            in_object = 1
            has_props = 0
            has_additional = 0
        }
        in_object && /properties:/ {
            has_props = 1
        }
        in_object && /additionalProperties:/ {
            has_additional = 1
        }
        in_object && /^[[:space:]]*}[,;]?[[:space:]]*$/ {
            if (has_props && !has_additional) {
                print "  Line " obj_line ": Missing additionalProperties"
            }
            in_object = 0
            has_props = 0
            has_additional = 0
        }
        ' "$file"
    fi
done
