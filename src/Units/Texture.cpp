#include "../Include.h"

#define STB_IMAGE_IMPLEMENTATION
#include "ImageLoader.h"

namespace smq {

	Texture::Texture(std::string Filename)
		: i_Filename(Filename)
	{
        glGenTextures(1, &i_ID);
        glBindTexture(GL_TEXTURE_2D, i_ID);

        stbi_set_flip_vertically_on_load(1);

        uint8_t* data = stbi_load(i_Filename.c_str(), &i_size.x, &i_size.y, &i_bpp, 4);

        if (!data) {
            Warn("Texture Load Failed, Missing Texture: " + i_Filename);

            i_Filename = "resources/textures/nTex.png";
            data = stbi_load(i_Filename.c_str(), &i_size.x, &i_size.y, &i_bpp, 4);

            if (!data) {
                smq::Error("Testure Load Failed Missing Missing Texture - nTex.png");
                return;
            }
        }

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);

        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA8, i_size.x, i_size.y, 0, GL_RGBA, GL_UNSIGNED_BYTE, data);

        stbi_image_free(data);
        Log("Texture Created Successfully: " + i_Filename);
	}

    Texture::Texture(unsigned int ID, Vector2Int size) 
		: i_ID(ID)
		, i_size(size)
    {
    
    }

	Texture::~Texture() {
		if (i_ID != 0) glDeleteTextures(1, &i_ID);
		Log("Texture Deleted Sucesfully");
	}

	bool Texture::Valid() {
		return i_ID != 0;
	}

	void Texture::ActivateTexture(unsigned int slot) {
		glActiveTexture(GL_TEXTURE0 + slot);
		glBindTexture(GL_TEXTURE_2D, i_ID);
	}

	Vector2Int Texture::Size() {
		return i_size;
	}
}