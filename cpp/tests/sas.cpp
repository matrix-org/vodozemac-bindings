#include "../../target/cxxbridge/vodozemac/src/lib.rs.h"
#include "gtest/gtest.h"

using namespace rust;

TEST(SasTest, Creation) {
  auto alice = sas::new_sas();
  auto bob = sas::new_sas();

  auto alice_key = alice->public_key()->to_base64();
  auto bob_key = bob->public_key()->to_base64();

  EXPECT_STRNE(alice_key.c_str(), bob_key.c_str());
}

TEST(SasTest, SharedSecret) {
  auto alice = sas::new_sas();
  auto bob = sas::new_sas();

  auto alice_key = alice->public_key();
  auto bob_key = bob->public_key();

  auto alice_established = alice->diffie_hellman(*bob_key);
  auto bob_established = bob->diffie_hellman(*alice_key);
}

TEST(SasTest, ShortAuthString) {
  auto alice = sas::new_sas();
  auto bob = sas::new_sas();

  auto alice_key = alice->public_key();
  auto bob_key = bob->public_key();

  auto alice_established = alice->diffie_hellman(*bob_key);
  auto bob_established = bob->diffie_hellman(*alice_key);

  auto alice_bytes = alice_established->bytes("");
  auto bob_bytes = bob_established->bytes("");

  ASSERT_EQ(alice_bytes->emoji_indices(), bob_bytes->emoji_indices());
  ASSERT_EQ(alice_bytes->decimals(), bob_bytes->decimals());
}

TEST(SasTest, CalculateMac) {
  auto alice = sas::new_sas();
  auto bob = sas::new_sas();

  auto alice_key = alice->public_key();
  auto bob_key = bob->public_key();

  auto alice_established = alice->diffie_hellman(*bob_key);
  auto bob_established = bob->diffie_hellman(*alice_key);

  auto alice_mac = alice_established->calculate_mac("Hello", "world");
  auto bob_mac = bob_established->calculate_mac("Hello", "world");

  ASSERT_STREQ(alice_mac->to_base64().c_str(), bob_mac->to_base64().c_str());

  EXPECT_NO_THROW(alice_established->verify_mac("Hello", "world", *bob_mac));
  EXPECT_NO_THROW(bob_established->verify_mac("Hello", "world", *alice_mac));
  EXPECT_ANY_THROW(bob_established->verify_mac("", "", *alice_mac));
}
