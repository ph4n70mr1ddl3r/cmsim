---
stepsCompleted: [1, 2, 3, 4, 5, 6]
inputDocuments:
  - '\\wsl.localhost\riddler\home\riddler\cppsim\docs\prd.md'
workflowType: 'architecture'
lastStep: 6
project_name: 'cppsim'
user_name: 'Riddler'
date: '2025-12-11T00:00:05+08:00'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**

The system implements a complete heads-up No-Limit Hold'em poker server with 64 functional requirements spanning nine domains. The core architectural pattern is an **authoritative server** model where all game state lives server-side, and clients are untrusted participants.

Key functional domains:
- **Connection lifecycle**: WebSocket handshake, session management, seat assignment, disconnection/reconnection handling
- **Game state machine**: Hand initialization, dealer button rotation, blind posting, betting rounds (preflop/flop/turn/river), hand evaluation, pot calculation including side pots
- **Player action validation**: In-turn enforcement, bet sizing rules, timeout detection (30s), legal action verification
- **Economy management**: 100 BB starting stacks, automatic top-up requests, play money unlimited reloads
- **Protocol layer**: JSON message serialization, WebSocket transport, protocol versioning, error messaging
- **Security boundary**: Input validation, malformed message rejection, rate limiting (1 action/100ms), malicious client detection (disconnect after 10 consecutive invalid actions)
- **Observability**: Dual-perspective logging architecture - server console logs all state changes/actions/errors, client console logs game view/decisions/observations
- **Bot simulation**: Random strategy agents with variable response times (1-3s human-like delays), automatic reconnection, reload requests on bust-out

**Non-Functional Requirements:**

**Performance Targets:**
- Action processing latency: < 50ms server-side for any valid poker action
- State broadcast latency: < 100ms total (serialization + transmission to both clients)
- Timeout granularity: < 1s detection for 30s action deadlines
- JSON serialization overhead: < 5ms per operation

**Reliability Requirements:**
- Extended uptime: 72+ hour continuous operation without crashes
- Stress testing: 10,000+ consecutive hands without failures or memory leaks
- State consistency: Graceful handling of all error conditions without game state corruption
- Deterministic execution: All state transitions reproducible from logs
- Success metric: **Zero crashes, zero rule violations, zero exploits**

**Security Requirements:**
- 100% message validation against JSON schema
- Zero-trust client model: Server maintains authoritative state, rejects all client-reported data
- Exploit resistance: Invalid actions, out-of-turn moves, illegal bet sizes, stack manipulation attempts all blocked
- Rate limiting enforcement and malicious behavior detection

**Testability & Observability:**
- Complete log traceability: Any hand reconstructable from dual-perspective logs alone
- Edge case coverage: All-in scenarios, disconnections during all phases, timeouts, malformed messages
- Performance metrics logging: Action processing time, state update latency visibility

### Scale & Complexity

- **Primary domain**: Backend Network Server (real-time game state management)
- **Complexity level**: Medium
  - Single-threaded design reduces concurrency complexity
  - Real-time networking and timeout management add moderate complexity
  - Full poker rules implementation (hand evaluation, pot calculation, side pots) requires domain expertise
  - Extensive validation and error handling logic
- **Estimated architectural components**: 6-8 major components
  - Network layer (WebSocket server, connection manager)
  - Protocol layer (JSON message handler, schema validator)
  - Game engine (state machine, NLHE rules, hand evaluator)
  - Player manager (session tracking, stack management)
  - Security layer (input validator, rate limiter, malicious behavior detector)
  - Logging subsystem (dual-perspective console output)
  - Bot client (strategy engine, network client, decision simulator)
  - Testing framework (edge case suite, stress test harness)

### Technical Constraints & Dependencies

**Architecture Constraints:**
- **Single-threaded execution model**: Mandatory for MVP to ensure deterministic behavior and eliminate race conditions
- **WebSocket+JSON protocol**: Required for future browser client compatibility (even though MVP uses C++ bots)
- **In-memory state only**: No database, no persistence layer in MVP
- **Synchronous game flow**: Sequential action processing on single event loop

**Technology Dependencies:**
- **C++17 or later**: Language requirement
- **Boost.Beast or uWebSockets**: WebSocket library
- **nlohmann/json or RapidJSON**: JSON serialization
- **Boost.Asio**: Async I/O and event loop
- **CMake**: Cross-platform build system
- **Google Test**: Unit testing framework

**Design Philosophy Constraints:**
- **Validation-first**: Extensive testing takes priority over feature velocity
- **Observability-first**: Comprehensive logging before optimization
- **Simplicity over performance**: Single-threaded design preferred for debuggability
- **Foundation quality**: Prove reliability before scaling to multi-table or mental poker protocol

### Cross-Cutting Concerns Identified

**1. State Consistency Management**
- All components must maintain single source of truth (server-side game state)
- State transitions must be atomic and logged
- Disconnection/timeout scenarios must preserve game integrity
- Affects: Game engine, network layer, player manager

**2. Security & Validation Layer**
- Every external input (WebSocket messages) must be validated before processing
- JSON schema enforcement at protocol boundary
- Action legality verification at game logic boundary
- Rate limiting and malicious behavior detection spanning network and game layers
- Affects: All layers that touch client input

**3. Dual-Perspective Logging**
- Server logs: Authoritative record of all actions, state changes, errors
- Client logs: Player perspective for game view validation
- Both perspectives required for complete hand traceability
- Affects: All components (every action must be logged from both viewpoints)

**4. Timeout & Disconnection Handling**
- 30-second action timeouts with < 1s detection
- 30-second reconnection grace period
- Graceful degradation (fold hand, preserve stack, award pot correctly)
- Affects: Network layer, game engine, state machine

**5. Error Handling Strategy**
- Connection errors: Disconnect, timeout, handshake failure
- Protocol errors: Malformed JSON, schema violations
- Game logic errors: Invalid actions, out-of-turn moves
- System errors: Memory allocation, file I/O
- All errors must be handled without crashes or state corruption
- Affects: All components (comprehensive error boundaries required)

**6. Testing & Verification Infrastructure**
- Edge case test suite (all poker scenarios, network failures, security exploits)
- Stress testing framework (10,000+ hand runs)
- Log analysis tools (verify correctness from dual-perspective logs)
- Performance measurement (latency tracking)
- Affects: Development workflow, component testability requirements

## Project Structure & Build System Evaluation

### Primary Technology Domain

**Backend Network Server** - C++ real-time poker simulation with WebSocket-based client-server architecture

### Technical Stack Selection

Based on project requirements and architectural constraints, the following technology decisions establish the foundation:

**Selected Technologies:**

| Category | Choice | Rationale |
|----------|--------|-----------|
| **Language** | C++17 | Modern C++ features (structured bindings, std::optional, if constexpr) while maintaining broad compiler support |
| **WebSocket Library** | Boost.Beast | Seamless integration with Boost.Asio event loop, header-only, well-maintained |
| **JSON Library** | nlohmann/json | Header-only, intuitive API, exceptional readability for dual-perspective logging |
| **Async I/O** | Boost.Asio | Industry-standard event loop, non-blocking I/O, timer support for timeouts |
| **Build System** | CMake 3.15+ | Cross-platform, modern targets, FetchContent support |
| **Dependency Management** | CMake FetchContent | Native CMake integration, reproducible builds, simple workflow |
| **Testing Framework** | Google Test | Mature C++ testing, mocking support (Google Mock), CMake integration |

### Project Structure Pattern

**Monorepo Architecture** - Single CMake project containing server, client, and shared libraries

**Rationale for Monorepo:**
- Shared code reuse (protocol definitions, game logic, JSON schemas)
- Synchronized versioning between server and client
- Simplified build and testing workflow
- Single source of truth for poker rules and message formats

**Proposed Directory Structure:**

```
cppsim/
├── CMakeLists.txt                 # Root CMake configuration
├── cmake/
│   ├── FetchDependencies.cmake    # Dependency management
│   └── CompilerWarnings.cmake     # Warning configurations
├── include/                       # Public headers (if creating library)
├── src/
│   ├── server/                   # Server executable
│   │   ├── main.cpp
│   │   ├── websocket_server.hpp/cpp
│   │   ├── connection_manager.hpp/cpp
│   │   ├── game_engine.hpp/cpp
│   │   ├── player_manager.hpp/cpp
│   │   └── logger.hpp/cpp
│   ├── client/                   # Bot client executable
│   │   ├── main.cpp
│   │   ├── websocket_client.hpp/cpp
│   │   ├── strategy_engine.hpp/cpp
│   │   └── client_logger.hpp/cpp
│   └── common/                   # Shared code
│       ├── protocol.hpp/cpp      # JSON message schemas
│       ├── poker_rules.hpp/cpp   # NLHE game logic
│       ├── hand_evaluator.hpp/cpp
│       └── types.hpp             # Common types and constants
├── tests/
│   ├── unit/                     # Google Test unit tests
│   ├── integration/              # Integration tests
│   └── stress/                   # Stress testing harness
└── docs/                         # Project documentation
```

### Architectural Decisions Provided by Build System

**Language & Runtime:**
- C++17 standard with compiler warnings as errors
- Cross-platform support (Linux, macOS, Windows)
- Optimization flags configurable per build type (Debug, Release, RelWithDebInfo)

**Dependency Management:**
- CMake FetchContent fetches Boost (Beast, Asio) and nlohmann/json at configure time
- Reproducible builds via version pinning in CMakeLists.txt
- No manual dependency installation required

**Build Tooling:**
- Modern CMake targets (interface libraries for header-only dependencies)
- Separate targets: `cppsim_server`, `cppsim_client`, `cppsim_tests`
- Shared library target: `cppsim_common` for protocol and game logic
- Build-time JSON schema validation support

**Testing Framework:**
- Google Test integrated via FetchContent
- Separate test executable: `cppsim_tests`
- CTest integration for `make test` / `ctest` execution
- Coverage reporting support (--coverage flags)

**Code Organization:**
- Header/implementation split for all non-trivial classes
- Common library prevents code duplication between server and client
- Clear separation: network layer, protocol layer, game logic, logging

**Development Experience:**
- CMake presets for common configurations (Debug, Release, WSL, etc.)
- Compiler warnings set to pedantic levels (-Wall, -Wextra, -Wpedantic)
- clang-format configuration for consistent code style
- VSCode/CLion CMake integration out-of-box

**Build Commands:**

```bash
# Configure (fetches dependencies)
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release

# Build all targets
cmake --build build

# Run tests
cd build && ctest --output-on-failure

# Build specific target
cmake --build build --target cppsim_server
```

### Alternatives Considered

**WebSocket Library Alternatives:**
- **uWebSockets**: Faster but requires more manual integration with async I/O, less documentation
- **websocketpp**: Older, less active maintenance compared to Boost.Beast

**Chosen**: Boost.Beast for Boost.Asio integration and maintainer support

**JSON Library Alternatives:**
- **RapidJSON**: Faster parsing but more complex API, harder to read logs
- **simdjson**: Extremely fast but read-only, no serialization

**Chosen**: nlohmann/json for developer experience and log readability (observability-first design)

**Dependency Management Alternatives:**
- **vcpkg**: Requires separate installation, binary caching benefits minimal for MVP
- **Conan**: Extra tooling dependency, overkill for 3 dependencies
- **Git submodules**: Manual updates, version drift risk

**Chosen**: CMake FetchContent for zero external tooling dependency

## Core Architectural Decisions

### Decision Priority Analysis

| Category | Decision | Criticality |
|----------|----------|-------------|
| **Architecture** | **Component-Based with DI** | Critical - Determines codebase structure and testability |
| **Protocol** | **Code-First (C++ Structs)** | Critical - Simplifies development workflow for C++ MVP |
| **State Machine** | **Functional FSM (std::variant)** | Critical - Guarantees compile-time state safety |
| **Validation** | **Gatekeeper Pattern** | Critical - Security boundary for Authoritative Server |
| **Observability** | **Centralized Event Bus** | Important - Ensures consistent, decoupled logging |

### Component Architecture

**Decision:** Component-Based Architecture with Dependency Injection
**Rationale:**
- **Flexibility**: Components (GameEngine, NetworkServer) are loosely coupled via interfaces.
- **Testability**: Dependencies can be easily mocked in unit tests (e.g., MockNetworkServer).
- **Modern C++**: Aligns with RAII and lifetime management best practices.
- **Alternatives Rejected**: Layered Architecture (too rigid), Singleton Pattern (hard to test).

### Network Protocol Design

**Decision:** Code-First C++ Structs + nlohmann macros
**Rationale:**
- **Simplicity**: No external code generation step required.
- **Reuse**: Shared `common/protocol.hpp` used by both client and server in Monorepo.
- **Type Safety**: Strongly typed C++ structs prevent serialization errors.
- **Alternatives Rejected**: Schema-First (tooling overhead), Manual JSON Parsing (bug-prone).

### Game Logic Architecture

**Decision:** Functional State Machine using `std::variant` + `std::visit`
**Rationale:**
- **Safety**: Invalid state transitions are impossible at compile-time.
- **Correctness**: Each state struct (Preflop, Flop) only contains data relevant to that phase.
- **Validation**: "Validation-First" philosophy enforced by type system.
- **Alternatives Rejected**: Enum+Switch (fragile), OOP State Pattern (dynamic allocation overhead).

### Security & Validation

**Decision:** Gatekeeper Pattern (Dedicated Validator)
**Rationale:**
- **Separation of Concerns**: `ActionValidator` component focuses purely on rules ("Can I bet?").
- **Security**: Validator acts as "bouncer" before `GameEngine` ever sees the action.
- **Testing**: Validation rules can be tested in isolation without setting up full game state.
- **Alternatives Rejected**: Embedded Validation (clutters business logic).

### Logging & Observability

**Decision:** Centralized Event Bus
**Rationale:**
- **Decoupling**: Business logic emits events; Loggers consume them.
- **Consistency**: Guarantees identical data stream for both Server Console and Client Console logs.
- **Traceability**: Event stream forms the "truth" of what happened.
- **Alternatives Rejected**: Direct Logger Calls (coupling, easy to miss logs).

### Decision Impact Analysis

**Implementation Sequence:**
1. **Core Infrastructure**: CMake setup, logging bus, `std::variant` state machine skeleton.
2. **Protocol Definition**: Shared `protocol.hpp` with JSON structs.
3. **Network Layer**: Basic Boost.Beast server/client handshake.
4. **Game Engine**: State machine implementation with Mock Network.
5. **Validation Layer**: Gatekeeper rules implementation.
6. **Integration**: Wiring it all together.

**Cross-Component Dependencies:**
- **Protocol** is the foundation for both **Network** and **Game Engine**.
- **Event Bus** must be available to ALL components for logging.
- **Game Engine** depends on **Validator** to approve actions.

## Implementation Patterns & Consistency Rules

### Pattern Overview

**Critical Conflict Points Identified:** 8 categories where AI agents could make different implementation choices, leading to inconsistent codebases.

### C++ Naming Conventions

**Type Naming (Classes, Structs, Enums):**
- `snake_case` for all types: `class game_engine`, `struct player_action`, `enum game_phase`
- Rationale: Consistency with STL/Boost (`std::vector`, `boost::asio`)

**Function & Method Naming:**
- `snake_case`: `void calculate_pot()`, `bool is_valid_bet()`
- Rationale: Matches standard library conventions

**Variable Naming:**
- `snake_case`: `int player_count`, `std::string session_id`
- Member variables: `player_stack_` (trailing underscore)
- Constants: `const int max_players = 2;` or `constexpr int MAX_PLAYERS = 2;` (UPPER_SNAKE for global constants)

**File Naming:**
- Headers: `game_engine.hpp`, `player_manager.hpp`
- Sources: `game_engine.cpp`, `player_manager.cpp`
- Tests: `game_engine_test.cpp`

### Error Handling Patterns

**Decision:** Explicit error handling using `std::optional`, `std::expected` (C++23), or custom `result<T, E>` type

**Rationale:**
- Forces explicit handling of every failure case
- Works cleanly with async callbacks (Boost.Asio handlers)
- Critical for "Validation-First" philosophy

**Pattern:**
```cpp
std::optional<player_action> parse_action(const json& msg);
// Returns std::nullopt on invalid JSON

result<game_state, validation_error> validate_action(const player_action& action);
// Explicit success/failure with error details
```

**Exceptions:**
- Reserved ONLY for truly exceptional situations (out-of-memory, logic errors)
- NOT used for validation failures or network errors

### Memory Management Patterns

**Ownership Model:** Unique Ownership Hierarchy

- Use `std::unique_ptr` for owned resources
- Use raw pointers (`T*`) or references (`T&`) for non-owning access
- `std::shared_ptr` ONLY when genuinely shared ownership (rare)

**Example:**
```cpp
class game_server {
    std::unique_ptr<game_engine> engine_;  // Server owns the engine
    std::unique_ptr<connection_manager> connections_;
};

// Pass non-owning references
void log_game_state(const game_engine& engine);
```

### Header & Include Patterns

**Header Guards:**
- Use `#pragma once` (simple, modern compilers support it)

**Include Order:**
1. Corresponding header (for `.cpp` files)
2. C system headers (`<cstdio>`)
3. C++ standard library (`<vector>`, `<string>`)
4. Third-party libraries (`<boost/asio.hpp>`, `<nlohmann/json.hpp>`)
5. Project headers (`"common/protocol.hpp"`)

**Example:**
```cpp
// game_engine.cpp
#include "game_engine.hpp"  // Own header first

#include <algorithm>
#include <vector>

#include <boost/asio.hpp>
#include <nlohmann/json.hpp>

#include "common/protocol.hpp"
#include "common/types.hpp"
```

### Modern C++ Usage

**auto keyword:**
- Use for complex iterator types: `auto it = map.find(key);`
- Use for obvious types: `auto engine = std::make_unique<game_engine>();`
- Avoid when type clarity matters: `int count = get_count();` (NOT `auto count`)

**const correctness:**
- Mark all non-mutating methods `const`
- Use `const&` for passing large objects
- Use `const` for variables that never change

**constexpr:**
- Use for compile-time constants: `constexpr int max_players = 2;`

### File Organization Patterns

**Header (.hpp) Files:**
- Class/struct declarations
- Inline functions (small, performance-critical)
- Template definitions (must be in header)

**Source (.cpp) Files:**
- Function implementations
- Static helper functions (local to file)

**Test Files:**
- Named `<component>_test.cpp`
- Located in `tests/unit/`
- One test file per component

### Event Bus & Logging Patterns

**Event Publishing:**
```cpp
// Components emit events, never call logger directly
event_bus.publish(player_bet_event{player_id, amount});
event_bus.publish(hand_complete_event{winner_id, pot_size});
```

**Logger Subscription:**
- `server_logger` subscribes to ALL events
- `client_logger` filters events by player visibility

### Testing Patterns

**Google Test Conventions:**
- Test names: `TEST(GameEngineTest, CalculatesPotCorrectly)`
- Test fixtures for setup/teardown: `class GameEngineFixture : public ::testing::Test`
- Use `EXPECT_*` for non-fatal assertions, `ASSERT_*` for fatal

**Mocking:**
- Use Google Mock for interfaces
- Mock external dependencies (network, time)
- Keep game logic pure and testable

### Enforcement Guidelines

**All AI Agents MUST:**
1. Follow snake_case naming for all C++ identifiers
2. Use explicit error handling (`std::optional`/`result<T,E>`), NOT exceptions for validation
3. Use `std::unique_ptr` for ownership, raw pointers/references for observation
4. Use `#pragma once` for header guards
5. Follow standard include ordering
6. Emit events to Event Bus, never call loggers directly
7. Write Google Test unit tests for all business logic

**Pattern Verification:**
- CI enforces clang-format configuration
- Code reviews check for explicit error handling
- Unit tests verify `const` correctness

## Project Structure & Boundaries

### Complete Project Directory Structure

```
cppsim/
├── CMakeLists.txt                      # Root CMake project
├── README.md                           # Project documentation
├── .gitignore                          # Git ignore patterns
├── .clang-format                       # Code formatting rules
├── .github/
│   └── workflows/
│       ├── build.yml                   # CI build pipeline
│       └── test.yml                    # CI test pipeline
│
├── cmake/
│   ├── FetchDependencies.cmake         # Boost, nlohmann/json, GTest
│   ├── CompilerWarnings.cmake          # Strict warning flags
│   └── CodeCoverage.cmake              # Coverage reporting
│
├── src/
│   ├── common/                         # Shared library (cppsim_common)
│   │   ├── CMakeLists.txt
│   │   ├── protocol.hpp                # JSON message schemas
│   │   ├── protocol.cpp
│   │   ├── poker_rules.hpp             # NLHE game logic
│   │   ├── poker_rules.cpp
│   │   ├── hand_evaluator.hpp          # Hand ranking algorithm
│   │   ├── hand_evaluator.cpp
│   │   ├── types.hpp                   # Common types (player_id, amount, etc.)
│   │   └── event_bus.hpp               # Event publishing/subscribing
│   │       └── event_bus.cpp
│   │
│   ├── server/                         # Server executable (cppsim_server)
│   │   ├── CMakeLists.txt
│   │   ├── main.cpp                    # Server entry point
│   │   ├── websocket_server.hpp        # Boost.Beast WebSocket server
│   │   ├── websocket_server.cpp
│   │   ├── connection_manager.hpp      # Manages client sessions
│   │   ├── connection_manager.cpp
│   │   ├── game_engine.hpp             # Core game state machine (std::variant FSM)
│   │   ├── game_engine.cpp
│   │   ├── player_manager.hpp          # Stack tracking, top-ups
│   │   ├── player_manager.cpp
│   │   ├── action_validator.hpp        # Gatekeeper validation
│   │   ├── action_validator.cpp
│   │   ├── server_logger.hpp           # Event bus subscriber for server logs
│   │   ├── server_logger.cpp
│   │   └── rate_limiter.hpp            # 1 action/100ms enforcement
│   │       └── rate_limiter.cpp
│   │
│   └── client/                         # Bot client executable (cppsim_client)
│       ├── CMakeLists.txt
│       ├── main.cpp                    # Bot entry point
│       ├── websocket_client.hpp        # Boost.Beast WebSocket client
│       ├── websocket_client.cpp
│       ├── strategy_engine.hpp         # Random strategy + timing simulation
│       ├── strategy_engine.cpp
│       └── client_logger.hpp           # Event bus subscriber for client perspective
│           └── client_logger.cpp
│
├── tests/
│   ├── CMakeLists.txt                  # Test executable (cppsim_tests)
│   ├── unit/
│   │   ├── poker_rules_test.cpp        # NLHE rules validation
│   │   ├── hand_evaluator_test.cpp     # Hand ranking correctness
│   │   ├── game_engine_test.cpp        # FSM transitions
│   │   ├── action_validator_test.cpp   # Validation rules
│   │   └── protocol_test.cpp           # JSON serialization
│   ├── integration/
│   │   ├── server_client_test.cpp      # End-to-end handshake
│   │   └── game_flow_test.cpp          # Full hand playthrough
│   └── stress/
│       └── load_test.cpp               # 10,000+ hands stress test
│
├── docs/
│   ├── prd.md                          # Product Requirements Document
│   ├── architecture.md                 # This document
│   └── protocol_spec.md                # JSON message specification
│
└── build/                              # CMake build output (gitignored)
    ├── bin/
    │   ├── cppsim_server
    │   ├── cppsim_client
    │   └── cppsim_tests
    └── lib/
        └── libcppsim_common.a
```

### Architectural Boundaries

**Component Boundaries:**
- **`common/` library**: Shared code (protocol, poker rules, event bus). Linked by both server and client.
- **`server/` executable**: Game logic, network server, validation. Single-threaded event loop.
- **`client/` executable**: Bot strategy, network client. Simulates player behavior.

**Event Bus Boundary:**
- Components emit events (`player_bet_event`, `hand_complete_event`) to `event_bus`.
- Loggers subscribe: `server_logger` (all events), `client_logger` (filtered by player visibility).
- NO direct logger calls from business logic.

**Validation Boundary (Gatekeeper):**
- JSON → `protocol::parse()` → `action_validator::validate()` → `game_engine::apply()`
- Invalid actions rejected at validator, never reach game engine.

### Requirements to Structure Mapping

**Functional Requirements → Implementation:**

| FR Category | Implementation Location |
|-------------|-------------------------|
| **Connection & Session** | `server/connection_manager.{hpp,cpp}`, `server/websocket_server.{hpp,cpp}` |
| **Poker Game Logic** | `common/poker_rules.{hpp,cpp}`, `server/game_engine.{hpp,cpp}` |
| **Player Actions** | `common/protocol.hpp`, `server/action_validator.{hpp,cpp}` |
| **Stack Management** | `server/player_manager.{hpp,cpp}` |
| **Network Protocol** | `common/protocol.{hpp,cpp}`, WebSocket server/client |
| **Security & Validation** | `server/action_validator.{hpp,cpp}`, `server/rate_limiter.{hpp,cpp}` |
| **Logging** | `common/event_bus.{hpp,cpp}`, `server/server_logger.{hpp,cpp}`, `client/client_logger.{hpp,cpp}` |
| **Bot Client** | `client/strategy_engine.{hpp,cpp}`, `client/websocket_client.{hpp,cpp}` |
| **Testing** | `tests/unit/`, `tests/integration/`, `tests/stress/` |

**Cross-Cutting Concerns:**

| Concern | Implementation |
|---------|----------------|
| **State Consistency** | `game_engine` FSM (std::variant), `event_bus` for audit trail |
| **Dual-Perspective Logging** | `event_bus` + `server_logger` + `client_logger` |
| **Timeout Handling** | Boost.Asio timers in `connection_manager` |
| **Error Handling** | `std::optional`/`result<T,E>` throughout codebase |

### Integration Points

**Internal Communication:**
- Server → Client: JSON messages over WebSocket (game state updates, action requests)
- Client → Server: JSON messages over WebSocket (actions, reload requests)
- Components → Event Bus: `event_bus.publish(event)`
- Event Bus → Loggers: `logger.on_event(event)` callbacks

**Build Integration:**
- CMake FetchContent: Downloads Boost, nlohmann/json, Google Test at configure time
- CTest: `ctest --output-on-failure` runs all tests

### Development Workflow Integration

**Build Commands:**
```bash
# Configure
cmake -B build -S . -DCMAKE_BUILD_TYPE=Debug

# Build all
cmake --build build

# Run server
./build/bin/cppsim_server

# Run client (in separate terminal)
./build/bin/cppsim_client

# Run tests
cd build && ctest --output-on-failure
```

**Adding New Component:**
1. Create `.hpp` and `.cpp` in appropriate directory (`server/`, `client/`, `common/`)
2. Add to `CMakeLists.txt` in that directory
3. Follow naming conventions (snake_case)
4. Create `_test.cpp` in `tests/unit/`

## Architecture Summary

### Key Architectural Decisions

This architecture document establishes the foundation for **cppsim**, a C++ poker server designed for reliability and future evolution to mental poker protocols.

**Core Architectural Patterns:**
1. **Component-Based Architecture with Dependency Injection** - Modular, testable components with explicit dependencies
2. **Functional State Machine (`std::variant`)** - Compile-time guarantees of state safety
3. **Gatekeeper Validation Pattern** - Security boundary separating validation from business logic
4. **Centralized Event Bus** - Decoupled observability with dual-perspective logging
5. **Code-First Protocol** - Type-safe JSON serialization using C++ structs

**Technology Stack:**
- **Language**: C++17 with modern idioms (auto, constexpr, structured bindings)
- **Libraries**: Boost.Beast (WebSocket), Boost.Asio (async I/O), nlohmann/json (serialization)
- **Build**: CMake 3.15+ with FetchContent for dependency management
- **Testing**: Google Test with CTest integration

**Design Philosophy:**
- **Validation-First**: Explicit error handling (`std::optional`, `result<T,E>`) forces handling of every failure case
- **Observability-First**: Event bus ensures complete dual-perspective audit trail
- **Simplicity Over Performance**: Single-threaded design eliminates concurrency bugs

**Implementation Consistency:**
- **Naming**: snake_case for all identifiers (types, functions, variables)
- **Memory**: Unique ownership (`std::unique_ptr`) with clear lifetime hierarchies
- **Headers**: `#pragma once`, standard include ordering
- **Testing**: Google Test for unit tests, one test file per component

This architecture provides a solid foundation for AI agents to implement the poker server with consistency and confidence, ensuring that code from multiple agents integrates seamlessly.
