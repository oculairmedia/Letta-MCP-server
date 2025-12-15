# Tool Validation Examples

This directory contains examples of common validation errors when creating tools for Letta.
These are intentionally incorrect to demonstrate what errors you'll encounter.

## Schema Validation Errors

### Missing Name Field
- File: `schema_missing_name.json`
- Error: Schema must have a "name" field

### Missing Parameters Field
- File: `schema_missing_parameters.json`
- Error: Schema must have a "parameters" field

### Missing Type in Parameters
- File: `schema_missing_type.json`
- Error: Parameters must specify type as "object"

### Missing Properties in Parameters
- File: `schema_missing_properties.json`
- Error: Parameters must have a "properties" object

### Property Without Description
- File: `schema_property_no_description.json`
- Error: Each property must have both "type" and "description"

### Property Without Type
- File: `schema_property_no_type.json`
- Error: Each property must have a "type" field

## Python Docstring Validation Errors

### Missing Docstring
- File: `python_no_docstring.py`
- Error: Functions must have a docstring

### Missing Args Section
- File: `python_no_args_section.py`
- Error: Docstring must have an "Args:" section

### Malformed Docstring
- File: `python_malformed_docstring.py`
- Error: Docstring must have proper triple quotes

## Valid Examples

### Complete Valid Tool
- Files: `valid_tool.py` and `valid_tool_schema.json`
- Shows proper format for both Python and schema files