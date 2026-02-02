#include "../Include.h"

namespace smq {

	RenderTexture::RenderTexture(Vector2Int size, Renderer* renderer, bool depthOnly)
        : i_size(size)
        , i_renderer(renderer)
    {
        glGenFramebuffers(1, &i_fbo);
        glBindFramebuffer(GL_FRAMEBUFFER, i_fbo);

        if (!depthOnly) {
            glGenTextures(1, &i_texture);
            glBindTexture(GL_TEXTURE_2D, i_texture);
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, i_size.x, i_size.y, 0, GL_RGB, GL_UNSIGNED_BYTE, nullptr);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
            glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, i_texture, 0);
        } else {
            glDrawBuffer(GL_NONE);
            glReadBuffer(GL_NONE);
        }

        glGenTextures(1, &i_textureDepth);
        glBindTexture(GL_TEXTURE_2D, i_textureDepth);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_DEPTH_COMPONENT, i_size.x, i_size.y, 0, GL_DEPTH_COMPONENT, GL_FLOAT, nullptr);

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

        if (depthOnly) {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER);
            float borderColor[] = { 1.0f, 1.0f, 1.0f, 1.0f };
            glTexParameterfv(GL_TEXTURE_2D, GL_TEXTURE_BORDER_COLOR, borderColor);
        } else {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
        }

        glFramebufferTexture2D(GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_TEXTURE_2D, i_textureDepth, 0);
	}

    RenderTexture::~RenderTexture() {}

    void RenderTexture::ActivateRenderTexture() {
        glBindFramebuffer(GL_FRAMEBUFFER, i_fbo);
        glViewport(0, 0, i_size.x, i_size.y);
    }

    void RenderTexture::Clear(bool color, bool depth) {
        glBindFramebuffer(GL_FRAMEBUFFER, i_fbo);

        GLbitfield mask = 0;
        if (color) mask |= GL_COLOR_BUFFER_BIT;
        if (depth) mask |= GL_DEPTH_BUFFER_BIT;

        if (mask != 0) glClear(mask);
    }

	bool RenderTexture::Valid() {
		return true;
	}

    void RenderTexture::Render(Matrix4 vp) {
        i_renderer->Render(eng.GetScene(), this, vp);
    }

    Vector2Int RenderTexture::GetSize() {
        return i_size;
    }

    unsigned int RenderTexture::GetTexture() {
        return i_texture;
    }

    unsigned int RenderTexture::GetTextureDepth() {
        return i_textureDepth;
    }
}