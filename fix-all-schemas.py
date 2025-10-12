#!/usr/bin/env python3
import re
import sys
from pathlib import Path

def add_additional_properties(content):
    """
    Add 'additionalProperties: false,' to object schemas that have properties but are missing it.
    """
    lines = content.split('\n')
    result = []
    i = 0

    while i < len(lines):
        line = lines[i]
        result.append(line)

        # Check if this line contains "type: 'object'" or 'type: "object"'
        if ("type: 'object'" in line or 'type: "object"' in line):
            # Look ahead to find if there's properties and additionalProperties
            has_properties = False
            has_additional_properties = False
            closing_brace_line = None
            brace_depth = 0
            j = i

            # Count opening braces on current line
            brace_depth += line.count('{') - line.count('}')

            # Scan forward
            for j in range(i + 1, min(i + 200, len(lines))):
                forward_line = lines[j]

                if 'properties:' in forward_line and brace_depth > 0:
                    has_properties = True

                if 'additionalProperties:' in forward_line:
                    has_additional_properties = True

                # Track brace depth
                brace_depth += forward_line.count('{') - forward_line.count('}')

                # Found the closing brace for our object
                if brace_depth == 0:
                    closing_brace_line = j
                    break

            # If we found properties but no additionalProperties, add it
            if has_properties and not has_additional_properties and closing_brace_line:
                # Skip forward to just before the closing brace
                for k in range(i + 1, closing_brace_line):
                    result.append(lines[k])

                # Get the indentation of the closing brace
                closing_line = lines[closing_brace_line]
                indent = len(closing_line) - len(closing_line.lstrip())
                indent_str = ' ' * (indent + 4)

                # Add additionalProperties before the closing brace
                result.append(f"{indent_str}additionalProperties: false,")
                result.append(closing_line)

                i = closing_brace_line + 1
                continue

        i += 1

    return '\n'.join(result)

def process_file(file_path):
    """Process a single JavaScript file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # Check if file has any object schemas
        if "type: 'object'" not in content and 'type: "object"' not in content:
            return False

        new_content = add_additional_properties(content)

        # Only write if changed
        if new_content != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            print(f"Fixed: {file_path}")
            return True

        return False

    except Exception as e:
        print(f"Error processing {file_path}: {e}", file=sys.stderr)
        return False

def main():
    tools_dir = Path('src/tools')

    if not tools_dir.exists():
        print("Error: src/tools directory not found", file=sys.stderr)
        sys.exit(1)

    js_files = list(tools_dir.rglob('*.js'))
    fixed_count = 0

    for js_file in js_files:
        if process_file(js_file):
            fixed_count += 1

    print(f"\nProcessed {len(js_files)} files, fixed {fixed_count} files")

if __name__ == '__main__':
    main()
