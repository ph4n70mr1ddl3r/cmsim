# Common library (shared protocol, poker rules, hand evaluator)
# For now, create as INTERFACE library since we have no sources yet
add_library(poker_common INTERFACE)

# Include directories
target_include_directories(poker_common
  INTERFACE
    ${CMAKE_CURRENT_SOURCE_DIR}
)

# Link dependencies (will be used in future stories)
target_link_libraries(poker_common
  INTERFACE
    nlohmann_json::nlohmann_json
)
