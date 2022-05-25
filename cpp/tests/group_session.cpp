#include "../../target/cxxbridge/vodozemac/src/lib.rs.h"
#include "gtest/gtest.h"

using namespace rust;

std::array<uint8_t, 32> PICKLE_KEY = {};

struct SessionCreationResult {
  Box<megolm::GroupSession> outbound;
  Box<megolm::InboundGroupSession> inbound;
};

SessionCreationResult create_session() {
  auto outbound = megolm::new_group_session();
  auto session_key = outbound->session_key();
  auto inbound = megolm::new_inbound_group_session(*session_key);

  auto ret = SessionCreationResult{
      std::move(outbound),
      std::move(inbound),
  };

  return ret;
}

TEST(GroupSessionTest, Creation) {
  auto [outbound, inbound] = create_session();

  auto outbound_id = outbound->session_id();
  auto inbound_id = inbound->session_id();

  EXPECT_NE(outbound_id.length(), 0);
  EXPECT_STREQ(outbound_id.c_str(), inbound_id.c_str());
}

TEST(GroupSessionTest, MessageIndex) {
  auto [outbound, inbound] = create_session();

  EXPECT_EQ(outbound->message_index(), 0);
  EXPECT_EQ(inbound->first_known_index(), 0);

  outbound->encrypt("Hello");
  auto inbound2 = megolm::new_inbound_group_session(*outbound->session_key());

  EXPECT_EQ(outbound->message_index(), 1);
  EXPECT_EQ(inbound2->first_known_index(), 1);
}

TEST(GroupSessionTest, Pickle) {
  auto session = megolm::new_group_session();

  auto pickle = session->pickle(PICKLE_KEY);
  auto unpickled = megolm::group_session_from_pickle(pickle, PICKLE_KEY);

  ASSERT_STREQ(session->session_id().c_str(), unpickled->session_id().c_str());
  EXPECT_EQ(session->message_index(), unpickled->message_index());
}

TEST(GroupSessionTest, PickleInbound) {
  auto [outbound, inbound] = create_session();

  auto pickle = inbound->pickle(PICKLE_KEY);
  auto unpickled =
      megolm::inbound_group_session_from_pickle(pickle, PICKLE_KEY);

  ASSERT_STREQ(inbound->session_id().c_str(), unpickled->session_id().c_str());
  EXPECT_EQ(inbound->first_known_index(), unpickled->first_known_index());
}

TEST(GroupSessionTest, UnpicklingFail) {
  EXPECT_ANY_THROW(megolm::group_session_from_pickle("", PICKLE_KEY));
  EXPECT_ANY_THROW(megolm::inbound_group_session_from_pickle("", PICKLE_KEY));
}

TEST(GroupSessionTest, DecryptionFail) {
  auto [outbound, inbound] = create_session();

  auto outbound2 = megolm::new_group_session();
  auto message = outbound2->encrypt("Hello");

  EXPECT_ANY_THROW(inbound->decrypt(*message));
}

TEST(GroupSessionTest, Encryption) {
  auto [outbound, inbound] = create_session();

  auto plaintext = "It's a secret to everybody";
  auto message = outbound->encrypt(plaintext);
  auto decrypted = inbound->decrypt(*message);

  EXPECT_STREQ(decrypted.plaintext.c_str(), plaintext);
  EXPECT_EQ(decrypted.message_index, 0);

  plaintext = "Another secret";
  message = outbound->encrypt(plaintext);
  decrypted = inbound->decrypt(*message);

  EXPECT_STREQ(decrypted.plaintext.c_str(), plaintext);
  EXPECT_EQ(decrypted.message_index, 1);
}

TEST(GroupSessionTest, SessionExport) {
  auto [outbound, inbound] = create_session();
  auto imported = megolm::import_inbound_group_session(*inbound->export_at(0));

  EXPECT_STREQ(outbound->session_id().c_str(), imported->session_id().c_str());

  auto plaintext = "It's a secret to everybody";
  auto message = outbound->encrypt(plaintext);
  auto decrypted = imported->decrypt(*message);

  EXPECT_STREQ(decrypted.plaintext.c_str(), plaintext);
  EXPECT_EQ(decrypted.message_index, 0);

  plaintext = "Another secret";
  message = outbound->encrypt(plaintext);
  decrypted = imported->decrypt(*message);

  EXPECT_STREQ(decrypted.plaintext.c_str(), plaintext);
  EXPECT_EQ(decrypted.message_index, 1);
}
