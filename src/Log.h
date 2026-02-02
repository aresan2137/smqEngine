#pragma once

#include <iostream>

namespace smq {

	inline void Log(std::string text) {
		std::cout << "Log: " << text << "\n";
	}

	inline void Error(std::string text) {
		std::cout << "Error: " << text << "\n";
		abort();
	}

	inline void Warn(std::string text) {
		std::cout << "Warn: " << text << "\n";
	}
}