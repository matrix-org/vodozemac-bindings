#include "../../target/cxxbridge/vodozemac/src/lib.rs.h"
#include "gtest/gtest.h"

using namespace rust;

std::array<uint8_t, 32> PICKLE_KEY = {};

struct SessionCreationResult {
  Box<olm::Account> alice;
  Box<olm::Account> bob;
  Box<olm::Session> session;
};

SessionCreationResult create_session() {
  Box<olm::Account> alice = olm::new_account();
  auto bob = olm::new_account();

  bob->generate_one_time_keys(1);

  auto one_time_keys = bob->one_time_keys();
  auto [key_id, one_time_key] = std::move(one_time_keys.front());

  auto identity_key = bob->curve25519_key();

  auto session = alice->create_outbound_session(*identity_key, *one_time_key);

  auto ret = SessionCreationResult{
      std::move(alice),
      std::move(bob),
      std::move(session),
  };

  return ret;
}

TEST(SessionTest, Creation) {
  auto [alice, bob, session] = create_session();

  auto session_id = session->session_id();

  EXPECT_NE(session_id.length(), 0);
}

TEST(SessionTest, IdUniqueness) {
  auto [alice1, bob1, session] = create_session();
  auto [alice2, bob2, session2] = create_session();

  auto session_id = session->session_id();
  auto session2_id = session2->session_id();

  EXPECT_STRNE(session_id.c_str(), session2_id.c_str());
}

TEST(SessionTest, Pickle) {
  auto [alice, bob, session] = create_session();

  auto pickle = session->pickle(PICKLE_KEY);
  auto unpickled = olm::session_from_pickle(pickle, PICKLE_KEY);

  auto session_id = session->session_id();
  auto session2_id = unpickled->session_id();

  EXPECT_STREQ(session_id.c_str(), session2_id.c_str());
}

TEST(SessionTest, InvalidPickle) {
  EXPECT_ANY_THROW(olm::session_from_pickle("", PICKLE_KEY));
}

TEST(SessionTest, Encryption) {
  auto [alice, bob, session] = create_session();

  auto alice_key = alice->curve25519_key();
  auto plaintext = "It's a secret to everybody";

  auto message = session->encrypt(plaintext);

  auto [bob_session, decrypted] =
      bob->create_inbound_session(*alice_key, *message);

  EXPECT_STREQ(session->session_id().c_str(),
               bob_session->session_id().c_str());

  EXPECT_STREQ(plaintext, decrypted.c_str());
}

TEST(SessionTest, InvalidDecryption) {
  auto parts = olm::OlmMessageParts{
      0,
      "",
  };

  EXPECT_ANY_THROW(olm::olm_message_from_parts(parts));
}

TEST(SessionTest, MultipleMessageDecryption) {
  auto [alice, bob, session] = create_session();

  auto alice_key = alice->curve25519_key();
  auto plaintext = "It's a secret to everybody";

  auto message = session->encrypt(plaintext);

  auto [bob_session, decrypted] =
      bob->create_inbound_session(*alice_key, *message);

  EXPECT_STREQ(session->session_id().c_str(),
               bob_session->session_id().c_str());

  EXPECT_STREQ(plaintext, decrypted.c_str());

  plaintext = "Grumble grumble";

  message = bob_session->encrypt(plaintext);
  decrypted = session->decrypt(*message);

  EXPECT_STREQ(plaintext, decrypted.c_str());
}

TEST(SessionTest, PreKeyMatches) {
  auto [alice, bob, session] = create_session();

  auto alice_key = alice->curve25519_key();
  auto plaintext = "It's a secret to everybody";

  auto message = session->encrypt(plaintext);

  auto [bob_session, decrypted] =
      bob->create_inbound_session(*alice_key, *message);

  plaintext = "Grumble grumble";
  message = session->encrypt(plaintext);

  EXPECT_TRUE(bob_session->session_matches(*message));
}

TEST(SessionTest, PreKeyDoesNotMatch) {
  auto [alice, bob, session] = create_session();
  auto [alice2, bob2, session2] = create_session();

  auto alice_key = alice->curve25519_key();
  auto plaintext = "It's a secret to everybody";

  auto message = session->encrypt(plaintext);

  auto [bob_session, decrypted] =
      bob->create_inbound_session(*alice_key, *message);

  plaintext = "Grumble grumble";
  message = session2->encrypt(plaintext);

  EXPECT_FALSE(bob_session->session_matches(*message));
}

TEST(SessionTest, InvalidOneTimeKey) {
  EXPECT_ANY_THROW(types::curve_key_from_base64(""));
}
