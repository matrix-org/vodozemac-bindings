rust-lib:
	cargo build

account-test: rust-lib tests/account.cpp
	g++ -Wall -g -pthread 		\
		tests/account.cpp 	\
		-lgtest_main -lgtest 	\
		-lpthread 		\
		-I ../target/cxxbridge 	\
		-L ../target/debug/ 	\
		-l vodozemac 		\
		-o account-test
	

test: account-test
	./account-test --gtest_color=yes
