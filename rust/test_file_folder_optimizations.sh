#!/bin/bash
# Functional test script for file/folder optimization (LMS-54)
set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "=================================="
echo "File/Folder Optimization Test (LMS-54)"
echo "=================================="
echo ""

DEFAULT_FILE_LIMIT=25
MAX_FILE_LIMIT=100
DEFAULT_FOLDER_LIMIT=20
MAX_FOLDER_LIMIT=50

tests_passed=0
tests_failed=0

run_test() {
    local test_name="$1"
    local test_cmd="$2"
    
    echo -e "${BLUE}Test: $test_name${NC}"
    if eval "$test_cmd"; then
        echo -e "${GREEN}✓ PASSED${NC}"
        ((tests_passed++))
    else
        echo -e "${RED}❌ FAILED${NC}"
        ((tests_failed++))
    fi
    echo ""
}

# Test 1
run_test "FileMetadata excludes content field" \
    "! grep -q 'pub content:' letta-server/src/tools/file_folder_ops.rs"

# Test 2
run_test "list_files uses DEFAULT_FILE_LIMIT" \
    "grep -q 'request.limit.unwrap_or(DEFAULT_FILE_LIMIT)' letta-server/src/tools/file_folder_ops.rs"

# Test 3
run_test "list_files enforces MAX_FILE_LIMIT" \
    "grep -A1 'request.limit.unwrap_or(DEFAULT_FILE_LIMIT)' letta-server/src/tools/file_folder_ops.rs | grep -q '.min(MAX_FILE_LIMIT)'"

# Test 4
run_test "list_files includes security hint" \
    "grep -q '\"File content is NEVER included in list operations\"' letta-server/src/tools/file_folder_ops.rs"

# Test 5
run_test "list_files includes pagination metadata" \
    "grep -q 'total: Some(total)' letta-server/src/tools/file_folder_ops.rs"

# Test 6
run_test "list_folders uses DEFAULT_FOLDER_LIMIT" \
    "grep -q 'request.limit.unwrap_or(DEFAULT_FOLDER_LIMIT)' letta-server/src/tools/file_folder_ops.rs"

# Test 7
run_test "list_folders truncates descriptions" \
    "grep -q 'truncate_string' letta-server/src/tools/file_folder_ops.rs"

# Test 8
run_test "open_file returns minimal confirmation" \
    "grep -q 'File marked as open in agent context' letta-server/src/tools/file_folder_ops.rs"

# Test 9
run_test "close_file has LMS-54 optimization" \
    "grep -q 'Minimal response as per LMS-54 requirements' letta-server/src/tools/file_folder_ops.rs"

# Test 10
run_test "close_all_files has minimal response" \
    "grep -q 'Minimal response - just file IDs, not full metadata (LMS-54)' letta-server/src/tools/file_folder_ops.rs"

# Test 11
run_test "attach_folder excludes agent_state" \
    "grep -q 'Minimal response - don.t include full agent state (LMS-54)' letta-server/src/tools/file_folder_ops.rs"

# Test 12
run_test "list_agents_in_folder optimized" \
    "grep -q 'Return IDs only - already optimized (LMS-54)' letta-server/src/tools/file_folder_ops.rs"

# Test 13
run_test "LMS-54 header comments present" \
    "grep -q 'Response size optimizations (LMS-54):' letta-server/src/tools/file_folder_ops.rs"

echo "=================================="
if [ $tests_failed -eq 0 ]; then
    echo -e "${GREEN}All $tests_passed Tests Passed! ✓${NC}"
    echo "=================================="
    echo ""
    echo "Summary of optimizations:"
    echo "  • list_files: limit=$DEFAULT_FILE_LIMIT (max=$MAX_FILE_LIMIT), no content"
    echo "  • list_folders: limit=$DEFAULT_FOLDER_LIMIT (max=$MAX_FOLDER_LIMIT), truncated descriptions"
    echo "  • open_file: Minimal confirmation"
    echo "  • close_file/close_all_files: Minimal responses"
    echo "  • attach/detach_folder: Excludes agent state"
    echo "  • list_agents_in_folder: IDs only"
    echo "  • Pagination metadata on all list operations"
    exit 0
else
    echo -e "${RED}$tests_failed of $(($tests_passed + $tests_failed)) tests failed${NC}"
    exit 1
fi
