#pragma once

#include <iostream>

inline void log(const std::string& text) {
	std::cout << "Log: " << text << "\n";
}

inline void warn(const std::string& text) {
	std::cout << "Warn: " << text << "\n";
	//__debugbreak();
}

inline void error(const std::string& text) {
	std::cout << "Error: " << text << "\n";
	__debugbreak();
	abort();
}