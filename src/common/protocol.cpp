#include "protocol.hpp"

#include <iostream>

namespace cppsim {
namespace protocol {

// Parse handshake message from JSON string
std::optional<handshake_message> parse_handshake(const std::string& json_str) {
  try {
    nlohmann::json j = nlohmann::json::parse(json_str);
    handshake_message msg;
    from_json(j, msg);
    return msg;
  } catch (const nlohmann::json::exception& e) {
    std::cerr << "Failed to parse handshake message: " << e.what() << std::endl;
    return std::nullopt;
  }
}

// Parse action message from JSON string
std::optional<action_message> parse_action(const std::string& json_str) {
  try {
    nlohmann::json j = nlohmann::json::parse(json_str);
    action_message msg;
    from_json(j, msg);
    return msg;
  } catch (const nlohmann::json::exception& e) {
    std::cerr << "Failed to parse action message: " << e.what() << std::endl;
    return std::nullopt;
  }
}

// Parse reload request message from JSON string
std::optional<reload_request_message> parse_reload_request(
    const std::string& json_str) {
  try {
    nlohmann::json j = nlohmann::json::parse(json_str);
    reload_request_message msg;
    from_json(j, msg);
    return msg;
  } catch (const nlohmann::json::exception& e) {
    std::cerr << "Failed to parse reload request message: " << e.what()
              << std::endl;
    return std::nullopt;
  }
}

// Parse disconnect message from JSON string
std::optional<disconnect_message> parse_disconnect(
    const std::string& json_str) {
  try {
    nlohmann::json j = nlohmann::json::parse(json_str);
    disconnect_message msg;
    from_json(j, msg);
    return msg;
  } catch (const nlohmann::json::exception& e) {
    std::cerr << "Failed to parse disconnect message: " << e.what()
              << std::endl;
    return std::nullopt;
  }
}

// Serialize state update message to JSON string
std::string serialize_state_update(const state_update_message& msg) {
  nlohmann::json j;
  to_json(j, msg);
  return j.dump();
}

// Serialize error message to JSON string
std::string serialize_error(const error_message& msg) {
  nlohmann::json j;
  to_json(j, msg);
  return j.dump();
}

// Serialize handshake response to JSON string
std::string serialize_handshake_response(const handshake_response& msg) {
  nlohmann::json j;
  to_json(j, msg);
  return j.dump();
}

// Serialize reload response to JSON string
std::string serialize_reload_response(const reload_response_message& msg) {
  nlohmann::json j;
  to_json(j, msg);
  return j.dump();
}

}  // namespace protocol
}  // namespace cppsim
