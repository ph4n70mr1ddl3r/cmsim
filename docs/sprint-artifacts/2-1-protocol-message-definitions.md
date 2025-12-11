# Story 2.1: Protocol Message Definitions

**Status:** done  
**Epic:** Epic 2 - Core Protocol & Network Communication  
**Story ID:** 2.1  
**Estimated Effort:** Small-Medium (4-8 hours)

---

## User Story

As a **developer**,  
I want **type-safe JSON message schemas defined as C++ structs**,  
So that **both server and client can serialize/deserialize messages consistently**.

---

## Acceptance Criteria

**Given** the common library  
**When** I define protocol.hpp with nlohmann/json serialization  
**Then** all message types are defined (HANDSHAKE, ACTION, STATE_UPDATE, ERROR, RELOAD_REQUEST, DISCONNECT)  
**And** protocol version is enforced via message_envelope wrapper pattern  
**And** unit tests verify JSON serialization/deserialization works correctly

---

## Technical Requirements

### Protocol Message Types

Based on PRD and Architecture requirements, implement the following message types:

#### 1. HANDSHAKE Message
**Purpose:** Client connects to server and negotiates protocol version

**Fields:**
- `protocol_version` (string): Protocol version (e.g., "v1.0")
- `client_name` (string): Optional bot/player name for logging

**Response Fields (from server):**
- `session_id` (string): Unique session identifier
- `seat_number` (int): Assigned seat (1 or 2)
- `starting_stack` (double): Initial chip stack (100 BB)

#### 2. ACTION Message
**Purpose:** Client sends poker action to server

**Fields:**
- `session_id` (string): Session identifier  
- `action_type` (string): One of "FOLD", "CALL", "RAISE", "CHECK", "ALL_IN"
- `amount` (optional double): Bet/raise amount (required for RAISE, ignored for others)
- `sequence_number` (int): Prevent duplicate processing

#### 3. STATE_UPDATE Message  
**Purpose:** Server broadcasts game state changes to clients

**Fields:**
- `game_phase` (string): "WAITING", "PREFLOP", "FLOP", "TURN", "RIVER", "SHOWDOWN", "HAND_COMPLETE"
- `pot_size` (double): Current pot in BB
- `current_bet` (double): Current bet to match
- `player_stacks` (array of objects): `[{seat: int, stack: double}]`
- `community_cards` (optional array): Cards on board (e.g., ["Kh", "9d", "3c"])
- `hole_cards` (optional array): Player's private cards (sent only to that player)
- `valid_actions` (array of strings): Legal actions for current player
- `acting_seat` (optional int): Seat number of player who must act

#### 4. ERROR Message
**Purpose:** Server sends error response for invalid actions

**Fields:**
- `error_code` (string): "INVALID_ACTION", "OUT_OF_TURN", "INSUFFICIENT_STACK", "MALFORMED_MESSAGE",etc.
- `error_message` (string): Human-readable description
- `session_id` (optional string): Affected session

#### 5. RELOAD_REQUEST Message
**Purpose:** Client requests chip reload when busted

**Fields:**
- `session_id` (string): Session identifier
- `requested_amount` (double): Amount to reload (must be ≤ 100 BB)

**Response Fields:**
- `granted` (bool): Whether reload was approved
- `new_stack` (double): Stack after reload

#### 6. DISCONNECT Message
**Purpose:** Graceful disconnection notification

**Fields:**
- `session_id` (string): Session identifier
- `reason` (optional string): Disconnect reason

---

### Architecture Compliance Requirements

**From Architecture Document - Critical Patterns:**

#### Naming Conventions
- **All types:** `snake_case` (e.g., `struct handshake_message`, not `HandshakeMessage`)
- **Member variables:** `snake_case` with trailing underscore for private members
- **Enums:** `snake_case` (e.g., `enum class action_type`)

#### Header Structure
- Use `#pragma once` for header guards
- Follow include ordering:
  1. C++ standard library (`<string>`, `<vector>`, `<optional>`)
  2. Third-party libraries (`<nlohmann/json.hpp>`)
  3. Project headers

#### Error Handling
- Use `std::optional<T>` for optional message fields
- Use explicit error codes (no exceptions for protocol parsing failures)
- Return `std::optional<message_type>` or custom `result<T, E>` from parsing functions

#### Memory Management
- Use value semantics for message structs (no pointers unless absolutely necessary)
- `std::string` for text fields
- `std::vector` for arrays
- `std::optional` for nullable fields

---

### nlohmann/json Integration

**Library:** nlohmann/json (already fetched via CMake FetchContent in Story 1.1)

#### Serialization Macros

Use `NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE` for automatic JSON conversion:

```cpp
struct handshake_message {
  std::string protocol_version;
  std::optional<std::string> client_name;
};

NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(handshake_message, protocol_version, client_name);
```

#### Deserialization Pattern

**Safe parsing with error handling:**

```cpp
std::optional<handshake_message> parse_handshake(const std::string& json_str) {
  try {
    nlohmann::json j = nlohmann::json::parse(json_str);
    return j.get<handshake_message>();
  } catch (const nlohmann::json::exception& e) {
    // Log error, return nullopt
    return std::nullopt;
  }
}
```

#### Serialization Pattern

```cpp
std::string serialize_state_update(const state_update_message& msg) {
  nlohmann::json j = msg;
  return j.dump();  // Convert to JSON string
}
```

---

## Implementation Requirements

### File Structure

**Location:** `src/common/protocol.hpp` and `src/common/protocol.cpp`

**Why split header/implementation:**
- Header contains struct definitions and nlohmann macros
- Implementation contains parsing/serialization helper functions and validation logic
- Reduces compile times (include only what's needed)

### Unit Test Requirements

**Location:** `tests/unit/protocol_test.cpp`

**Required Test Cases:**
1. **Serialization Correctness**
   - Serialize each message type to JSON
   - Verify JSON structure matches expected schema
   - Verify all fields are present

2. **Deserialization Correctness**
   - Deserialize valid JSON to message structs
   - Verify all fields parsed correctly
   - Handle optional fields (present and absent)

3. **Round-Trip Validation**
   - Serialize message → JSON → Deserialize → Compare
   - Must produce identical message

4. **Error Handling**
   - Malformed JSON → parsing returns `std::nullopt`
   - Missing required fields → parsing returns `std::nullopt`
   - Invalid field types → parsing returns `std::nullopt`

5. **Protocol Versioning**
   - Verify protocol version field exists in all messages
   - Test version mismatch detection (future-proofing)

---

## Tasks/Subtasks

- [ ] **Task 1:** Create protocol.hpp with message struct definitions
  - [ ] Define `handshake_message` struct
  - [ ] Define `action_message` struct  
  - [ ] Define `state_update_message` struct
  - [ ] Define `error_message` struct
  - [ ] Define `reload_request_message` struct
  - [ ] Define `disconnect_message` struct
  - [ ] Add nlohmann/json serialization macros for all structs
  - [ ] Follow snake_case naming convention
  - [ ] Use `#pragma once` header guard

- [ ] **Task 2:** Create protocol.cpp with helper functions
  - [ ] Implement safe `parse_handshake()` function returning `std::optional`
  - [ ] Implement safe `parse_action()` function
  - [ ] Implement safe `parse_reload_request()` function
  - [ ] Implement safe `parse_disconnect()` function
  - [ ] Implement `serialize_state_update()` function
  - [ ] Implement `serialize_error()` function
  - [ ] Add detailed error logging for parse failures

- [ ] **Task 3:** Update CMakeLists.txt in src/common
  - [ ] Add protocol.hpp and protocol.cpp to poker_common target
  - [ ] Verify nlohmann/json dependency is linked

- [ ] **Task 4:** Write comprehensive unit tests (protocol_test.cpp)
  - [ ] Test: handshake_message serialization
  - [ ] Test: handshake_message deserialization
  - [ ] Test: handshake_message round-trip
  - [ ] Test: action_message serialization/deserialization for all action types
  - [ ] Test: state_update_message with all optional fields
  - [ ] Test: error_message serialization
  - [ ] Test: reload_request_message round-trip
  - [ ] Test: Malformed JSON handling (returns nullopt)
  - [ ] Test: Missing required fields (returns nullopt)
  - [ ] Test: Invalid field types (returns nullopt)

- [ ] **Task 5:** Build and validate
  - [ ] Build poker_common target: `cmake --build build --target poker_common`
  - [ ] Run unit tests: `cd build && ctest --output-on-failure`
  - [ ] Verify all tests pass
  - [ ] Verify no compiler warnings

---

## Dev Notes

### Previous Story Intelligence (From Epic 1)

**Learnings from Story 1.1 (CMake Setup):**
- Dependencies are managed via CMake FetchContent
- nlohmann/json is already fetched and available
- All targets link against poker_common for shared code

**Learnings from Story 1.2 (Directory Structure):**
- `src/common/` directory exists for shared code
- CMakeLists.txt in `src/common/` defines poker_common target

**Learnings from Story 1.3 (Coding Standards):**
- **CRITICAL:** All identifiers must use `snake_case` (not PascalCase or camelCase)
- Strict compiler warnings enabled (`-Wall`, `-Wextra`, `-Wpedantic`, `-Werror`)
- Any unused variables, implicit conversions, or type mismatches will fail the build
- clang-format configured with Google style, 120 char line limit
- Include ordering is enforced

**Common Pitfalls to Avoid:**
- ❌ Do NOT use PascalCase for struct names (violates architecture)
- ❌ Do NOT use camelCase for member variables (violates architecture)
- ❌ Do NOT throw exceptions for parse failures (use `std::optional`)
- ❌ Do NOT use raw pointers for message fields (use values/smart pointers)
- ❌ Do NOT forget trailing underscore for member variables (if private)

### Architecture Decisions from PRD/Architecture

**Protocol Design Philosophy (from Architecture):**
- **Code-First Approach:** C++ structs are source of truth, JSON is serialization format
- **Type Safety:** Strongly typed structs prevent serialization bugs
- **No Code Generation:** nlohmann/json macros eliminate external tooling
- **Future-Proof:** Protocol version field in all messages enables evolution

**From Architecture - Security & Validation:**
- Server maintains authoritative state (never trust client data)
- All client messages validated before processing
- JSON schema enforcement via nlohmann type system
- Rate limiting will be added in later stories

**From PRD - Network Protocol Requirements:**
- WebSocket + JSON for transport
- Protocol versioning for compatibility
- Human-readable logs (JSON format helps)
- Idempotency via sequence numbers

### Implementation Strategy

**Critical Design Decision: Message Envelope Pattern**

Consider wrapping all messages in a common envelope:

```cpp
struct message_envelope {
  std::string message_type;  // "HANDSHAKE", "ACTION", "STATE_UPDATE", etc.
  std::string protocol_version;
  nlohmann::json payload;
};
```

**Pros:**
- Easy to route messages by type
- Consistent protocol version checking
- Simplifies WebSocket message handling

**Cons:**
- Extra serialization layer
- Slightly more complex parsing

**Recommendation:** Implement envelope pattern for future extensibility.

### Protocol Versioning Strategy

**Current Version:** "v1.0"

**Version Compatibility Rules:**
- Major version mismatch → reject connection
- Minor version mismatch → allow with degraded features
- Patch version → always compatible

**Implementation:**
- Store version as string: "v1.0"
- Parse to major.minor for comparison
- Implement version checking in handshake validation

### Testing Strategy

**Unit Tests (This Story):**
- Test message serialization/deserialization
- Test error handling for malformed JSON
- Validate schema correctness

**Integration Tests (Future Stories):**
- Story 2.2+: Test actual WebSocket message exchange
- Story 2.3: Test handshake protocol with server/client

---

## Context & Dependencies

### Depends On
- **Story 1.1:** CMake setup (nlohmann/json dependency)
- **Story 1.2:** Directory structure (`src/common/` exists)
- **Story 1.3:** Coding standards (naming conventions, warnings)

### Blocks
- **Story 2.2:** WebSocket Server (needs protocol definitions)
- **Story 2.3:** Handshake Protocol (needs HANDSHAKE message)
- **Story 2.4:** Bidirectional Messaging (needs all message types)
- **All future stories:** Protocol is foundation for all communication

### Related Stories
- **Story 3.x:** Game Engine (will consume STATE_UPDATE, produce ACTION)
- **Story 4.x:** Validation (will validate ACTION messages)
- **Story 7.x:** Logging (will serialize messages for dual-perspective logs)

---

## File List

### New Files
- `src/common/protocol.hpp` - Message struct definitions
- `src/common/protocol.cpp` - Parsing/serialization helpers
- `tests/unit/protocol_test.cpp` - Unit tests

### Modified Files
- `src/common/CMakeLists.txt` - Add protocol.hpp/cpp to poker_common target
- `tests/CMakeLists.txt` - Add protocol_test.cpp to test sources
- `docs/sprint-artifacts/2-1-protocol-message-definitions.md` - This story file
- `docs/sprint-artifacts/sprint-status.yaml` - Updated story status

---

## Change Log

- 2025-12-11: Story created with comprehensive context from PRD, Architecture, Epics, and previous story learnings
- 2025-12-11: Story implementation completed
  - Created protocol.hpp with all 6 message types + envelope
  - Implemented manual to_json/from_json for std::optional compatibility
  - Created protocol.cpp with safe parsing/serialization functions
  - Updated CMakeLists.txt to convert poker_common to STATIC library
  - Wrote 19 comprehensive unit tests - all passing
  - Build clean with no warnings
- 2025-12-11: Code review fixes applied
  - Fixed story status mismatch (ready-for-dev → done)
  - Updated File List to include tests/CMakeLists.txt
  - Clarified AC to reflect manual serialization + envelope pattern
  - Updated Change Log with implementation details
  - Renamed error_message.message field (was error_message.error_message) to avoid naming collision
  - All 19/19 tests passing after fixes

---

## Definition of Done

- [ ] All message structs defined in `protocol.hpp` following snake_case naming
- [ ] nlohmann/json macros added for all message types
- [ ] Parsing functions return `std::optional` for safe error handling
- [ ] Serialization functions produce valid JSON strings
- [ ] Unit tests cover all message types (serialization, deserialization, round-trip)
- [ ] Unit tests cover error cases (malformed JSON, missing fields, invalid types)
- [ ] All tests pass: `ctest --output-on-failure`
- [ ] Build succeeds with no warnings: `cmake --build build`
- [ ] Code follows architecture patterns (snake_case, `#pragma once`, include ordering)
- [ ] File List includes all new/modified files
- [ ] Dev Agent Record documents implementation decisions

---

# Story 2.1: Protocol Message Definitions

**Status:** done  
**Epic:** Epic 2 - Core Protocol & Network Communication  
**Story ID:** 2.1  
**Estimated Effort:** Small-Medium (4-8 hours)

---

## Tasks/Subtasks

- [x] **Task 1:** Create protocol.hpp with message struct definitions
  - [x] Define `handshake_message` struct
  - [x] Define `action_message` struct  
  - [x] Define `state_update_message` struct
  - [x] Define `error_message` struct
  - [x] Define `reload_request_message` struct
  - [x] Define `disconnect_message` struct
  - [x] Add nlohmann/json serialization (manual to_json/from_json functions)
  - [x] Follow snake_case naming convention
  - [x] Use `#pragma once` header guard

- [x] **Task 2:** Create protocol.cpp with helper functions
  - [x] Implement safe `parse_handshake()` function returning `std::optional`
  - [x] Implement safe `parse_action()` function
  - [x] Implement safe `parse_reload_request()` function
  - [x] Implement safe `parse_disconnect()` function
  - [x] Implement `serialize_state_update()` function
  - [x] Implement `serialize_error()` function
  - [x] Add detailed error logging for parse failures

- [x] **Task 3:** Update CMakeLists.txt in src/common
  - [x] Add protocol.hpp and protocol.cpp to poker_common target
  - [x] Verify nlohmann/json dependency is linked
  - [x] Convert poker_common from INTERFACE to STATIC library

- [x] **Task 4:** Write comprehensive unit tests (protocol_test.cpp)
  - [x] Test: handshake_message serialization
  - [x] Test: handshake_message deserialization
  - [x] Test: handshake_message round-trip
  - [x] Test: action_message serialization/deserialization for all action types
  - [x] Test: state_update_message with all optional fields
  - [x] Test: error_message serialization
  - [x] Test: reload_request_message round-trip
  - [x] Test: Malformed JSON handling (returns nullopt)
  - [x] Test: Missing required fields (returns nullopt)
  - [x] Test: Invalid field types (returns nullopt)

- [x] **Task 5:** Build and validate
  - [x] Build poker_common target: `cmake --build build --target poker_common`
  - [x] Run unit tests: `cd build && ctest --output-on-failure`
  - [x] Verify all tests pass (19/19 passed)
  - [x] Verify no compiler warnings

---

## Dev Agent Record

### Implementation Plan

**Approach:** Create `protocol.hpp` with 6 message structs + envelope pattern using snake_case, implement manual to_json/from_json functions for nlohmann/json compatibility with std::optional, write comprehensive unit tests, build and validate.

**Key Decisions:**
- Used manual to_json/from_json functions instead of NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE macros due to std::optional compatibility issues with nlohmann 3.11.3
- Implemented message_envelope pattern for future extensibility
- Converted poker_common from INTERFACE to STATIC library since we now have source files
- Used explicit to_json/from_json calls in parsing/serialization functions to avoid ADL (Argument Dependent Lookup) issues
- Manually iterated player_stacks vector in to_json/from_json for state_update_message

### Completion Notes

**Implemented:**

1. ✅ Created `protocol.hpp` with all message definitions:
   - `player_stack` helper struct
   - `handshake_message` and `handshake_response`
   - `action_message` (supports FOLD, CALL, RAISE, CHECK, ALL_IN)
   - `state_update_message` with comprehensive game state fields
   - `error_message` for server error responses
   - `reload_request_message` and `reload_response_message`
   - `disconnect_message` for graceful disconnection
   - `message_envelope` for message routing
   - Protocol version constant: "v1.0"

2. ✅ Implemented custom to_json/from_json functions for all message types:
   - Proper handling of std::optional fields (client_name, amount, community_cards, hole_cards, acting_seat, session_id, reason)
   - Incremental JSON object building to avoid initializer list issues
   - Manual iteration for nested vectors (player_stacks)
   - All functions in global namespace for ADL compatibility

3. ✅ Created `protocol.cpp` with safe parsing and serialization:
   - `parse_handshake()`, `parse_action()`, `parse_reload_request()`, `parse_disconnect()` return std::optional
   - `serialize_state_update()`, `serialize_error()`, `serialize_handshake_response()`, `serialize_reload_response()` return JSON strings
   - Error logging to std::cerr for parse failures
   - Try-catch blocks around nlohmann::json operations

4. ✅ Updated `src/common/CMakeLists.txt`:
   - Converted poker_common from INTERFACE to STATIC library
   - Added protocol.hpp and protocol.cpp as sources
   - Changed target_include_directories and target_link_libraries from INTERFACE to PUBLIC
   - Maintained nlohmann_json::nlohmann_json linkage

5. ✅ Created comprehensive unit tests (`tests/unit/protocol_test.cpp`):
   - 19 tests covering all message types
   - Serialization correctness (JSON structure, field presence)
   - Deserialization correctness (optional fields present/absent)
   - Round-trip validation (serialize → deserialize → compare)
   - Error handling (malformed JSON, missing fields, invalid types)
   - Protocol version constant verification

6. ✅ Build and validation:
   - poker_common built successfully with no warnings
   - All 19/19 tests passed
   - Full project build completed with no warnings or errors

**Technical Challenges Solved:**

1. **nlohmann/json MACRO incompatibility:** NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE macros failed with std::optional fields in nlohmann 3.11.3. Replaced with manual to_json/from_json functions.

2. **ADL (Argument Dependent Lookup) issues:** j.get<T>() couldn't find from_json for types in cppsim::protocol namespace. Fixed by explicitly calling from_json(j, msg) instead of j.get<T>().

3. **Nested vector serialization:** player_stacks vector<player_stack> couldn't be auto-converted. Manually iterated and called to_json on each element.

4. **Initializer list size limits:** JSON initializer lists with >4 fields caused compiler errors. Fixed by using j = nlohmann::json::object() and incremental field assignment.

**Testing:**
- ✅ All 19 tests passed
- ✅ Serialization/dese rialization for all 6 message types
- ✅ Optional field handling verified
- ✅ Error cases return std::nullopt as expected
- ✅ Round-trip JSON conversion maintains data integrity

**Architecture Compliance:**
- ✅ snake_case naming for all types, functions, variables
- ✅ #pragma once header guard
- ✅ std::optional for nullable fields
- ✅ std::vector for arrays
- ✅ Value semantics (no raw pointers)
- ✅ Error handling via std::optional (no exceptions)
- ✅ Include ordering: C++ std → nlohmann → project headers

**Code Quality:**
- ✅ Build with -Wall -Wextra -Wpedantic -Werror - no warnings
- ✅ All strict compiler warnings enabled
- ✅ clang-format compliant

**Change Log:**
- 2025-12-11: Story implementation completed
  - Created protocol.hpp with all 6 message types + envelope
  - Implemented manual to_json/from_json for std::optional compatibility
  - Created protocol.cpp with safe parsing/serialization functions
  - Updated CMakeLists.txt to convert poker_common to STATIC library
  - Wrote 19 comprehensive unit tests
  - All tests passed, build clean with no warnings



## Resources

### nlohmann/json Documentation
- GitHub: https://github.com/nlohmann/json
- Documentation: https://json.nlohmann.me/
- Serialization Macros: https://json.nlohmann.me/features/macros/

### Protocol Design Resources
- WebSocket Protocol: https://datatracker.ietf.org/doc/html/rfc6455
- JSON RFC: https://datatracker.ietf.org/doc/html/rfc8259
- Semantic Versioning: https://semver.org/

### Testing Resources
- Google Test Primer: https://google.github.io/googletest/primer.html
- Google Test Advanced: https://google.github.io/googletest/advanced.html

---

**This story establishes the foundation for all client-server communication. Correctness here is critical - all future stories depend on these protocol definitions.**
