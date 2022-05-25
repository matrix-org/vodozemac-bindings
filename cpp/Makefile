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
		-ldl			\
		-o account-test
	

session-test: rust-lib tests/session.cpp
	g++ 				\
		-Wall -g 		\
		-pthread -std=c++17	\
		tests/session.cpp 	\
		-lgtest_main -lgtest 	\
		-lpthread 		\
		-I ../target/cxxbridge 	\
		-L ../target/debug/ 	\
		-l vodozemac 		\
		-ldl			\
		-o session-test

sas-test: rust-lib tests/sas.cpp
	g++ 				\
		-Wall -g 		\
		-pthread -std=c++17	\
		tests/sas.cpp 		\
		-lgtest_main -lgtest 	\
		-lpthread 		\
		-I ../target/cxxbridge 	\
		-L ../target/debug/ 	\
		-l vodozemac 		\
		-ldl			\
		-o sas-test

group-session-test: rust-lib tests/group_session.cpp
	g++ 				\
		-Wall -g 		\
		-pthread -std=c++17	\
		tests/group_session.cpp	\
		-lgtest_main -lgtest 	\
		-lpthread 		\
		-I ../target/cxxbridge 	\
		-L ../target/debug/ 	\
		-l vodozemac 		\
		-ldl			\
		-o group-session-test

test: account-test session-test sas-test group-session-test
	./account-test --gtest_color=yes
	./session-test --gtest_color=yes
	./sas-test --gtest_color=yes
	./group-session-test --gtest_color=yes
