#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function fixSchemaFile(filePath) {
    let content = fs.readFileSync(filePath, 'utf8');
    let modified = false;

    // Pattern to match: type: 'object', followed by properties:, but no additionalProperties:
    // before the closing brace
    const lines = content.split('\n');
    const newLines = [];

    for (let i = 0; i < lines.length; i++) {
        newLines.push(lines[i]);

        // Check if this line contains "type: 'object'"
        if (lines[i].includes("type: 'object'") || lines[i].includes('type: "object"')) {
            // Look ahead to find properties and check for additionalProperties
            let hasProperties = false;
            let hasAdditionalProperties = false;
            let closingBraceIndex = -1;
            let braceDepth = 0;
            let startedObject = false;

            for (let j = i; j < Math.min(i + 100, lines.length); j++) {
                const line = lines[j];

                // Count braces to track depth
                const openBraces = (line.match(/{/g) || []).length;
                const closeBraces = (line.match(/}/g) || []).length;

                if (j === i) {
                    // First line might have opening brace
                    if (line.includes('{')) {
                        startedObject = true;
                        braceDepth = 1;
                    }
                } else {
                    braceDepth += openBraces - closeBraces;
                }

                if (line.includes('properties:')) {
                    hasProperties = true;
                }

                if (line.includes('additionalProperties:')) {
                    hasAdditionalProperties = true;
                }

                // Check if we've closed the object
                if (startedObject && braceDepth === 0 && closeBraces > 0) {
                    closingBraceIndex = j;
                    break;
                }
            }

            // If we found properties but no additionalProperties, add it
            if (hasProperties && !hasAdditionalProperties && closingBraceIndex > 0) {
                // Find the line with the closing brace
                const closingLine = lines[closingBraceIndex];
                const indent = closingLine.match(/^(\s*)/)[1];

                // Insert additionalProperties before the closing brace
                const additionalPropsLine = `${indent}    additionalProperties: false,`;

                // We need to insert this line before the closing brace
                // Skip ahead to where we'll insert it
                let insertIndex = closingBraceIndex;
                for (let k = i + 1; k < closingBraceIndex; k++) {
                    newLines.push(lines[k]);
                }

                newLines.push(additionalPropsLine);
                modified = true;
                i = closingBraceIndex - 1; // Continue from after the object
            }
        }
    }

    if (modified) {
        fs.writeFileSync(filePath, newLines.join('\n'), 'utf8');
        console.log(`Fixed: ${filePath}`);
        return true;
    }

    return false;
}

// Find all JS files in src/tools
function findJSFiles(dir) {
    const files = [];
    const items = fs.readdirSync(dir);

    for (const item of items) {
        const fullPath = path.join(dir, item);
        const stat = fs.statSync(fullPath);

        if (stat.isDirectory()) {
            files.push(...findJSFiles(fullPath));
        } else if (item.endsWith('.js')) {
            files.push(fullPath);
        }
    }

    return files;
}

// Main
const toolsDir = path.join(__dirname, 'src', 'tools');
const files = findJSFiles(toolsDir);

let fixedCount = 0;
for (const file of files) {
    if (fixSchemaFile(file)) {
        fixedCount++;
    }
}

console.log(`\nFixed ${fixedCount} files`);
