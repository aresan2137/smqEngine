#pragma once
#include "units.h"

class RenderTexture {
public:
    uint32_t FBO;
    uint32_t textureID;
    uint32_t positionTextureID = 0;
    uint32_t NormalTextureID = 0;
    uint32_t rbo;
    Vector2Int size;
    bool depthOnly;
    bool hasPosition;

    RenderTexture(Vector2Int _size, bool _depthOnly = false, bool _withPosition = false, uint32_t _filter = GL_NEAREST, uint32_t _wrap = GL_REPEAT)
        : size(_size)
        , depthOnly(_depthOnly)
        , hasPosition(_withPosition) {
        glGenFramebuffers(1, &FBO);
        glBindFramebuffer(GL_FRAMEBUFFER, FBO);

        glGenTextures(1, &textureID);
        glBindTexture(GL_TEXTURE_2D, textureID);

        if (!depthOnly) {
            glTexStorage2D(GL_TEXTURE_2D, 1, GL_RGBA8, size.x, size.y);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, _filter);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, _filter);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, _wrap);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, _wrap);
            glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, textureID, 0);

            if (hasPosition) {
                glGenTextures(1, &positionTextureID);
                glBindTexture(GL_TEXTURE_2D, positionTextureID);
                glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA32F, size.x, size.y, 0, GL_RGBA, GL_FLOAT, NULL);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
                glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT1, GL_TEXTURE_2D, positionTextureID, 0);

                glGenTextures(1, &NormalTextureID);
                glBindTexture(GL_TEXTURE_2D, NormalTextureID);
                glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F, size.x, size.y, 0, GL_RGBA, GL_FLOAT, NULL);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
                glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT2, GL_TEXTURE_2D, NormalTextureID, 0);

                uint32_t attachments[3] = {
                    GL_COLOR_ATTACHMENT0,
                    GL_COLOR_ATTACHMENT1,
                    GL_COLOR_ATTACHMENT2
                };

                glDrawBuffers(3, attachments);
            }

            glGenRenderbuffers(1, &rbo);
            glBindRenderbuffer(GL_RENDERBUFFER, rbo);
            glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH24_STENCIL8, size.x, size.y);
            glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_RENDERBUFFER, rbo);
        } else {
            glTexImage2D(GL_TEXTURE_2D, 0, GL_DEPTH_COMPONENT, size.x, size.y, 0, GL_DEPTH_COMPONENT, GL_FLOAT, NULL);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, _filter);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, _filter);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, _wrap);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, _wrap);
            glFramebufferTexture2D(GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_TEXTURE_2D, textureID, 0);

            glDrawBuffer(GL_NONE);
            glReadBuffer(GL_NONE);
        }

        if (glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE) {
            error("RenderTexture Error: Framebuffer is not complete!");
        }

        glBindFramebuffer(GL_FRAMEBUFFER, 0);
    };

    ~RenderTexture() {
        glDeleteFramebuffers(1, &FBO);
        glDeleteTextures(1, &textureID);
        if (hasPosition && positionTextureID != 0) {
            glDeleteTextures(1, &positionTextureID);
        }
        if (!depthOnly) {
            glDeleteRenderbuffers(1, &rbo);
        }
        if (NormalTextureID != 0) {
            glDeleteTextures(1, &NormalTextureID);
        }
    };

    void activateRenderTexture() {
        glBindFramebuffer(GL_FRAMEBUFFER, FBO);
        glViewport(0, 0, size.x, size.y);
    };

    void unbind(Vector2Int windowSize) {
        glBindFramebuffer(GL_FRAMEBUFFER, 0);
        glViewport(0, 0, windowSize.x, windowSize.y);
    };

    uint32_t getTexture() {
        return textureID;
    };

    void clean() {
        if (depthOnly) {
            glClear(GL_DEPTH_BUFFER_BIT);
        } else {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }
    };

    RenderTexture(const RenderTexture&) = delete;
    RenderTexture& operator=(const RenderTexture&) = delete;
};