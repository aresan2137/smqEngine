#pragma once

#include "Units.h"
#include <fstream> 

#include "Log.h"

namespace smq {
	inline std::string LoadFile(std::string Filename) {
        std::ifstream file(Filename, std::ios::binary);

        if (!file) Error("LoadFile() file loading failed"); 

        std::string content(
            (std::istreambuf_iterator<char>(file)),
            std::istreambuf_iterator<char>()
        );

        if (content.size() >= 3 &&
            (unsigned char)content[0] == 0xEF &&
            (unsigned char)content[1] == 0xBB &&
            (unsigned char)content[2] == 0xBF) {
            content.erase(0, 3);
        }

        return content;
	}
}