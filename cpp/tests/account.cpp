#include "../../target/cxxbridge/vodozemac/src/lib.rs.h"
#include "gtest/gtest.h"

using namespace rust;

std::array<uint8_t, 32> PICKLE_KEY = {};

TEST(AccountTest, AccountCreation) {
  auto alice = olm::new_account();
  auto key = alice->ed25519_key();
  auto encoded_key = key->to_base64();

  EXPECT_NE(encoded_key.length(), 0);
}

TEST(AccountTest, OneTimeKeyGeneration) {
  auto alice = olm::new_account();

  EXPECT_EQ(alice->one_time_keys().size(), 0);

  alice->generate_one_time_keys(10);
  EXPECT_EQ(alice->one_time_keys().size(), 10);

  alice->mark_keys_as_published();
  EXPECT_EQ(alice->one_time_keys().size(), 0);
}

TEST(AccountTest, FallbackKeyGeneration) {
  auto alice = olm::new_account();
  EXPECT_EQ(alice->fallback_key().size(), 0);

  alice->generate_fallback_key();
  EXPECT_EQ(alice->fallback_key().size(), 1);

  alice->mark_keys_as_published();
  EXPECT_EQ(alice->one_time_keys().size(), 0);
}

TEST(AccountTest, MaxKeysTest) {
  auto alice = olm::new_account();
  auto max_keys = alice->max_number_of_one_time_keys();

  EXPECT_GT(max_keys, 0);
  EXPECT_LT(max_keys, 1000);
}

TEST(AccountTest, PickleTest) {
  auto alice = olm::new_account();

  auto pickle = alice->pickle(PICKLE_KEY);

  auto unpickled = olm::account_from_pickle(pickle, PICKLE_KEY);

  EXPECT_EQ(alice->curve25519_key()->to_base64(),
            unpickled->curve25519_key()->to_base64());
}
