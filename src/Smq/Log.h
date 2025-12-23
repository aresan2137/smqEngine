#pragma once

#include <iostream>

namespace smq {

	inline void Log(std::string text) {
		std::cout << "Log: " << text << "\n";
	}

	inline void Error(std::string text, bool Abort = true) {
		std::cout << "Error: " << text << "\n";
		if (Abort) abort();
	}

	inline void Warn(std::string text) {
		std::cout << "Warn: " << text << "\n";
	}
}