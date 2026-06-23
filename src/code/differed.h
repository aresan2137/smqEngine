#pragma once

#include "smq.h"

Mesh* baked;

struct MeshInfoGPU {
    uint32_t indexStart;
    uint32_t indexEnd;
    float pad0[2];

    glm::vec3 aabbMin;
    float pad1;

    glm::vec3 aabbMax;
    float pad2;

    glm::mat4 invModelMatrix;
};

struct LightGPU {
    glm::vec3 pos;
    float power;

    glm::vec3 color;
    float maxReach;

    glm::vec3 aabbMin;
    float pad0;

    glm::vec3 aabbMax;
    float pad1;
};

GLuint meshSSBO = 0;
GLuint lightSSBO = 0;

void UploadMeshes(const std::vector<MeshInfoGPU>& meshes) {
    if (!meshSSBO)
        glGenBuffers(1, &meshSSBO);

    glBindBuffer(GL_SHADER_STORAGE_BUFFER, meshSSBO);
    glBufferData(GL_SHADER_STORAGE_BUFFER,
        meshes.size() * sizeof(MeshInfoGPU),
        meshes.data(),
        GL_DYNAMIC_DRAW);
}

void UploadLights(const std::vector<LightGPU>& lights) {
    if (!lightSSBO)
        glGenBuffers(1, &lightSSBO);

    glBindBuffer(GL_SHADER_STORAGE_BUFFER, lightSSBO);
    glBufferData(GL_SHADER_STORAGE_BUFFER,
        lights.size() * sizeof(LightGPU),
        lights.data(),
        GL_DYNAMIC_DRAW);
}

uint32_t LoadComputeShader(const std::string& computeCode) {
    const char* src = computeCode.c_str();

    uint32_t cs = glCreateShader(GL_COMPUTE_SHADER);
    glShaderSource(cs, 1, &src, NULL);
    glCompileShader(cs);

    int success;
    char log[1024];

    glGetShaderiv(cs, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(cs, 1024, NULL, log);
        error(std::string("CS compile error:\n") + log);
    }

    uint32_t prog = glCreateProgram();
    glAttachShader(prog, cs);
    glLinkProgram(prog);

    glGetProgramiv(prog, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(prog, 1024, NULL, log);
        error(std::string("CS link error:\n") + log);
    }

    glDeleteShader(cs);
    return prog;
}

Color inline KelvinToColor(float Kelvin) {
    Color color = { 255,255,255,255 };

    Kelvin /= 100;

    if (Kelvin <= 66) {
        color.r = 255;
    } else {
        color.r = Kelvin - 60;
        color.r = 329.698727446 * pow(color.r, -0.1332047592);
        if (color.r < 0) {
            color.r = 0;
        }
        if (color.r > 255) {
            color.r = 255;
        }
    }

    if (Kelvin <= 66) {
        color.g = Kelvin;
        color.g = 99.4708025861 * log(color.g) - 161.1195681661;
        if (color.g < 0) {
            color.g = 0;
        }
        if (color.g > 255) {
            color.g = 255;
        }
    } else {
        color.g = Kelvin - 60;
        color.g = 288.1221695283 * pow(color.g, -0.0755148492);
        if (color.g < 0) {
            color.g = 0;
        }
        if (color.g > 255) {
            color.g = 255;
        }
    }

    if (Kelvin >= 66) {
        color.b = 255;
    } else {
        if (Kelvin <= 19) {
            color.b = 0;
        } else {
            color.b = Kelvin - 10;
            color.b = 138.5177312231 * log(color.b) - 305.0447927307;
            if (color.b < 0) {
                color.b = 0;
            }
            if (color.b > 255) {
                color.b = 255;
            }
        }
    }

    return color;
}

class DifferedDrawer : public System {
private:
    uint32_t computeShaderID;

public:
    DifferedDrawer(uint32_t id) : computeShaderID(id) {}

    void Update(Scene* scene) override {
        if (scene->cameras.empty())
            return;

        Camera& cam = scene->cameras[0];
        Vector2Int windowSize = cam.renderTexture->size;

        cam.renderTexture->activateRenderTexture();
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        // ---- geometry pass ----
        drawObject(scene->rootObject, cam.getVP(), glm::mat4(1.0f));

        cam.renderTexture->unbind(windowSize);

        // ---- compute lighting ----
        if (computeShaderID != 0 && cam.renderTexture->hasPosition) {
            glUseProgram(computeShaderID);

            // textures
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, cam.renderTexture->positionTextureID);

            glActiveTexture(GL_TEXTURE3);
            glBindTexture(GL_TEXTURE_2D, cam.renderTexture->NormalTextureID);

            glActiveTexture(GL_TEXTURE2);
            glBindTexture(GL_TEXTURE_2D, gssf<Texture>(ssf::lut)->ID);

            // output image
            glBindImageTexture(
                1,
                cam.renderTexture->textureID,
                0, GL_FALSE, 0,
                GL_READ_WRITE,
                GL_RGBA8
            );

            // old mesh buffers (VBO/EBO)
            uint32_t meshVBO = baked->vbo;
            uint32_t meshEBO = baked->ibo;

            glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 4, meshVBO);
            glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 5, meshEBO);

            // NEW SSBO
            glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 6, meshSSBO);
            glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 7, lightSSBO);

            GLuint gx = (GLuint)ceil(windowSize.x / 8.0f);
            GLuint gy = (GLuint)ceil(windowSize.y / 8.0f);

            glDispatchCompute(gx, gy, 1);
            glMemoryBarrier(GL_SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }

    void drawObject(Object* obj, Matrix4 VP, Matrix4 Model) {
        if (obj->hasComponent<Position3D>()) {
            auto* pos = obj->getComponent<Position3D>();
            Model = Model
                * glm::translate(glm::mat4(1.0f), pos->position)
                * glm::mat4_cast(pos->rotation)
                * glm::scale(glm::mat4(1.0f), pos->scale);
        }

        if (obj->hasComponent<ModelMaterial>()) {
            auto* mm = obj->getComponent<ModelMaterial>();

            mm->material->updateUniform(0, VP * Model);
            mm->material->updateUniform(1, Model);

            mm->material->activateMaterial();
            mm->mesh->activateMesh();

            glDrawElements(GL_TRIANGLES,
                mm->mesh->getIndexCount(),
                GL_UNSIGNED_INT,
                0);
        }

        for (auto* child : obj->kids)
            drawObject(child, VP, Model);
    }
};