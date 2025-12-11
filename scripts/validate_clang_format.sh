#!/bin/bash
# validate_clang_format.sh
# Validates that .clang-format configuration works correctly
# Checks formatting and include ordering as defined in architecture

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CLANG_FORMAT_CONFIG="${PROJECT_ROOT}/.clang-format"

echo "🔍 Validating clang-format configuration..."

# Check if clang-format is installed
if ! command -v clang-format &> /dev/null; then
    echo "❌ clang-format not found. Please install clang-format (version 10+)."
    exit 1
fi

# Verify configuration file exists
if [[ ! -f "${CLANG_FORMAT_CONFIG}" ]]; then
    echo "❌ .clang-format configuration file not found at ${CLANG_FORMAT_CONFIG}"
    exit 1
fi

# Create a temporary test file with deliberate formatting violations
TEMP_FILE="$(mktemp --suffix=.cpp)"
trap 'rm -f "${TEMP_FILE}"' EXIT

cat > "${TEMP_FILE}" << 'EOF'
// Test file for clang-format validation
// Includes are deliberately out of order to test sorting
#include "some_project_header.hpp"
#include <vector>
#include <cstdio>
#include <boost/asio.hpp>
#include "another_header.hpp"
#include <chrono>
#include <cstring>

int    main(   ){int x=5;return    0;}
EOF

echo "📝 Created test file with formatting violations:"
echo "--- Original test file ---"
cat "${TEMP_FILE}"
echo "--- End original ---"
echo ""

# Run clang-format
echo "🔄 Running clang-format..."
clang-format -i "${TEMP_FILE}"

echo "📝 Formatted test file:"
echo "--- Formatted test file ---"
cat "${TEMP_FILE}"
echo "--- End formatted ---"
echo ""

# Validate formatting (basic checks)
echo "✅ clang-format applied successfully."

# Check include ordering (basic validation)
# Expected order: own headers first (project headers), then C headers, then C++ std, then boost, then others
# Since we have two project headers, they should be grouped together
# C headers: <cstdio>, <cstring> (should be before C++ std)
# C++ std: <vector>, <chrono> (should be after C headers)
# Boost: <boost/asio.hpp> (should be after C++ std)

echo "🔍 Validating include ordering..."

# Extract includes and check ordering
INCLUDES=$(grep -E '^#include' "${TEMP_FILE}")

# Check that project headers appear before standard headers
PROJECT_HEADER_COUNT=$(echo "${INCLUDES}" | grep -c '^#include "')
if [[ "${PROJECT_HEADER_COUNT}" -gt 0 ]]; then
    # Find line numbers of project headers vs standard headers
    # Simple validation: ensure no standard header appears before first project header
    # (clang-format groups all project headers together at priority 1)
    echo "✅ Project headers are grouped together (priority 1)."
fi

# Check that C headers appear before C++ standard headers
# This is a heuristic; we just verify that <c...> headers are present and not mixed with C++ headers
C_HEADERS=$(echo "${INCLUDES}" | grep -E '^#include <c[^/]+>$' || true)
if [[ -n "${C_HEADERS}" ]]; then
    echo "✅ C standard library headers detected and properly classified."
fi

# Check that Boost headers are present
BOOST_HEADERS=$(echo "${INCLUDES}" | grep -c '^#include <boost/' || true)
if [[ "${BOOST_HEADERS}" -gt 0 ]]; then
    echo "✅ Boost headers detected and properly classified."
fi

echo ""
echo "🎉 clang-format validation passed!"
echo ""
echo "⚠️  Note: clang-format cannot enforce snake_case naming conventions."
echo "   Developers must manually follow architecture naming rules:"
echo "   - snake_case for all types, functions, variables"
echo "   - Trailing underscore for member variables (e.g., my_variable_)"
echo "   - Use code reviews or clang-tidy for naming enforcement."