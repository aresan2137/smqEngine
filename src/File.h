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

    inline void SaveFile(std::string path, std::string content) {
        std::ofstream file(path);

        if (file.is_open()) {
            const char bom[] = { (char)0xEF, (char)0xBB, (char)0xBF };
            file.write(bom, 3);

            file << content;

            file.close();
            Log("FileSave() saved sucesfully");
        } else {
            Error("FileSave() failed to save");
        }
    }
}