#pragma once

#include "units.h"

#include "stb_image.h"

class Texture {
public:
    enum class SamplingMode {
        Nearest = GL_NEAREST,
        Bilinear = GL_LINEAR
	};

    enum class SampleOverflow {
        Clamp = GL_CLAMP_TO_BORDER,
        Repeat = GL_REPEAT
    };

    uint32_t ID = 0;

    Texture(const std::string& path, SamplingMode mode = SamplingMode::Nearest, SampleOverflow overflow = SampleOverflow::Repeat) {
        int width, height, channels;
        stbi_set_flip_vertically_on_load(true);
        unsigned char* data = stbi_load(path.c_str(), &width, &height, &channels, STBI_rgb_alpha);

        if (!data) {
            error("Failed to load texture: " + path);
            return;
        }

        glCreateTextures(GL_TEXTURE_2D, 1, &ID);

        glTextureStorage2D(ID, 1, GL_RGBA8, width, height);

        glTextureSubImage2D(ID, 0, 0, 0, width, height, GL_RGBA, GL_UNSIGNED_BYTE, data);

        glTextureParameteri(ID, GL_TEXTURE_MIN_FILTER, static_cast<uint32_t>(mode));
        glTextureParameteri(ID, GL_TEXTURE_MAG_FILTER, static_cast<uint32_t>(mode));
        glTextureParameteri(ID, GL_TEXTURE_WRAP_S, static_cast<uint32_t>(overflow));
        glTextureParameteri(ID, GL_TEXTURE_WRAP_T, static_cast<uint32_t>(overflow));

        stbi_image_free(data);
    }

    Texture(const std::vector<uint8_t>& buffer, SamplingMode mode = SamplingMode::Nearest, SampleOverflow overflow = SampleOverflow::Repeat) {
        int width, height, channels;
        stbi_set_flip_vertically_on_load(true);

        unsigned char* data = stbi_load_from_memory(
            buffer.data(),
            (int)buffer.size(),
            &width, &height, &channels,
            STBI_rgb_alpha
        );

        if (!data) {
            error("Failed to load texture from memory buffer");
            return;
        }

        glCreateTextures(GL_TEXTURE_2D, 1, &ID);
        glTextureStorage2D(ID, 1, GL_RGBA8, width, height);
        glTextureSubImage2D(ID, 0, 0, 0, width, height, GL_RGBA, GL_UNSIGNED_BYTE, data);

        glTextureParameteri(ID, GL_TEXTURE_MIN_FILTER, static_cast<uint32_t>(mode));
        glTextureParameteri(ID, GL_TEXTURE_MAG_FILTER, static_cast<uint32_t>(mode));
        glTextureParameteri(ID, GL_TEXTURE_WRAP_S, static_cast<uint32_t>(overflow));
        glTextureParameteri(ID, GL_TEXTURE_WRAP_T, static_cast<uint32_t>(overflow));

        stbi_image_free(data);
    }

    Texture(Color color) {
        glCreateTextures(GL_TEXTURE_2D, 1, &ID);

        glTextureStorage2D(ID, 1, GL_RGBA8, 1, 1);

        glTextureSubImage2D(ID, 0, 0, 0, 1, 1, GL_RGBA, GL_UNSIGNED_BYTE, &color);

        glTextureParameteri(ID, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
        glTextureParameteri(ID, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
        glTextureParameteri(ID, GL_TEXTURE_WRAP_S, GL_REPEAT);
        glTextureParameteri(ID, GL_TEXTURE_WRAP_T, GL_REPEAT);
    }

    Texture(Vector2Int size, int format, SamplingMode mode = SamplingMode::Nearest, SampleOverflow overflow = SampleOverflow::Repeat) {
        glCreateTextures(GL_TEXTURE_2D, 1, &ID);

        glTextureStorage2D(ID, 1, format, size.x, size.y);

        glTextureParameteri(ID, GL_TEXTURE_MIN_FILTER, static_cast<uint32_t>(mode));
        glTextureParameteri(ID, GL_TEXTURE_MAG_FILTER, static_cast<uint32_t>(mode));
        glTextureParameteri(ID, GL_TEXTURE_WRAP_S, static_cast<uint32_t>(overflow));
        glTextureParameteri(ID, GL_TEXTURE_WRAP_T, static_cast<uint32_t>(overflow));
    }

    ~Texture() {
        if (ID != 0) glDeleteTextures(1, &ID);
    }

    void UpdateSampler(SamplingMode mode, SampleOverflow overflow) {
        glTextureParameteri(ID, GL_TEXTURE_MIN_FILTER, static_cast<uint32_t>(mode));
        glTextureParameteri(ID, GL_TEXTURE_MAG_FILTER, static_cast<uint32_t>(mode));
        glTextureParameteri(ID, GL_TEXTURE_WRAP_S, static_cast<uint32_t>(overflow));
        glTextureParameteri(ID, GL_TEXTURE_WRAP_T, static_cast<uint32_t>(overflow));
    }

    void ActivateTexture(uint32_t slot = 0) {
        glBindTextureUnit(slot, ID);
    }

    void ActivateTexture(uint32_t unit, int format, uint32_t access = GL_READ_WRITE) {
        glBindImageTexture(unit, ID, 0, GL_FALSE, 0, access, format);
    }
};