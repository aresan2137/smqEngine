#pragma once

#include "Units.h"

namespace smq {
	inline uint64_t ToTag(std::string string) {
		if (string.size() > 8) {
			Warn("Tag string too long, It's going to be cut off");
			string = string.substr(0, 8);
		}

		uint64_t out = 0;
		for (int i = 0; i < 8 && string[i] != '\0'; ++i) {
			out |= (static_cast<uint64_t>(string[i]) << (i * 8));
		}
		return out;
	}

	inline std::string FromTag(uint64_t tag) {
		std::string result = "";
		for (int i = 0; i < 8; ++i) {
			char c = static_cast<char>((tag >> (i * 8)) & 0xFF);
			if (c == '\0') break;
			result += c;
		}
		return result;
	}
}