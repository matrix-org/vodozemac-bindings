rust-lib:
	cargo build

account-test: rust-lib tests/account.cpp
	g++ 				\
		-Wall -g 		\
		-pthread -std=c++17	\
		tests/account.cpp 	\
		-lgtest_main -lgtest 	\
		-lpthread 		\
		-I ../target/cxxbridge 	\
		-L ../target/debug/ 	\
		-l vodozemac 		\
		-o account-test
	

session-test: rust-lib tests/account.cpp
	g++ 				\
		-Wall -g 		\
		-pthread -std=c++17	\
		tests/session.cpp 	\
		-lgtest_main -lgtest 	\
		-lpthread 		\
		-I ../target/cxxbridge 	\
		-L ../target/debug/ 	\
		-l vodozemac 		\
		-o session-test

test: account-test session-test
	./account-test --gtest_color=yes
	./session-test --gtest_color=yes
