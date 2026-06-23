#pragma once

#include <variant>
#include <vector>

using SSFasset = std::variant<Mesh*, Texture*, std::string*, std::vector<uint8_t>*>;

enum class ssf : size_t {
#include "../assets/game_layout.txt"
};

inline std::vector<SSFasset> assets;

template <typename T>
inline T* gssf(ssf id) {
    return std::get<T*>(assets[static_cast<size_t>(id)]);
}

std::vector<SSFasset> LoadSSF(const std::string& path) {
    std::ifstream file(path, std::ios::binary);

    std::vector<SSFasset> out;

    if (!file.is_open()) {
        error("ssf file not found: " + path);
        return {};
    }

    char magic[3];
    file.read(magic, 3);
    uint8_t version;
    file.read((char*)&version, 1);

    if (magic[0] != 'S' || magic[1] != 'S' || magic[2] != 'F') {
        error("provided ssf file but it's header isn't ssf's");
        return {};
    }

    if (version != 1) {
        error("unsupported ssf version: " + std::to_string(version));
        return {};
    }

    while (file.peek() != EOF) {
        uint8_t type;
        file.read((char*)&type, 1);

        if (type == 0xFF) {
            break;
        }

        uint32_t size;
        file.read((char*)&size, sizeof(uint32_t));

        std::vector<uint8_t> buffer(size);
        file.read((char*)buffer.data(), size);

        if (type == 0x01) {
            log("loading custo bin not suporrted " + std::to_string(size));
        } else if (type == 0x02) {
            out.push_back(new Texture(buffer));
        } else if (type == 0x03) {
			out.push_back(new Mesh(buffer));
        } else if (type == 0x04) {
			out.push_back(new std::string(buffer.begin(), buffer.end()));
        } else if (type == 0x05) {
            out.push_back(new std::vector<uint8_t>(buffer));
        } else {
            warn("Unknown file type in SSF: " + std::to_string(type));
        }
    }

    file.close();
    return out;
}
