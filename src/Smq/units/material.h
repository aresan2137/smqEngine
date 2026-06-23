#pragma once

#include "units.h"
#include "Mesh.h"
#include "Texture.h"


Shader inline LoadShader(const std::string& vertexCode, const std::string& fragmentCode) {

    const char* vShaderSource = vertexCode.c_str();
    const char* fShaderSource = fragmentCode.c_str();

    uint32_t vertex = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertex, 1, &vShaderSource, NULL);
    glCompileShader(vertex);

    int success;
    char infoLog[1024];
    glGetShaderiv(vertex, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(vertex, 1024, NULL, infoLog);
        error("Vertex Shader Compilation Error:\n" + std::string(infoLog));
    }

    uint32_t fragment = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragment, 1, &fShaderSource, NULL);
    glCompileShader(fragment);

    glGetShaderiv(fragment, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(fragment, 1024, NULL, infoLog);
        error("Fragment Shader Compilation Error:\n" + std::string(infoLog));
    }

    Shader ID = glCreateProgram();
    glAttachShader(ID, vertex);
    glAttachShader(ID, fragment);
    glLinkProgram(ID);

    glGetProgramiv(ID, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(ID, 1024, NULL, infoLog);
        error("Shader Linking Error:\n" + std::string(infoLog));
    }

    glDeleteShader(vertex);
    glDeleteShader(fragment);

    return ID;
}

class Material {
public:
    Shader shader;

    Material(Shader _shader) : shader(_shader) {};

    void updateUniform(uint32_t location, float data) {
        glProgramUniform1f(shader, location, data);
    };

    void updateUniform(uint32_t location, Vector2 data) {
        glProgramUniform2f(shader, location, data.x, data.y);
    };

    void updateUniform(uint32_t location, Vector3 data) {
        glProgramUniform3f(shader, location, data.x, data.y, data.z);
    };

    void updateUniform(uint32_t location, Vector4 data) {
        glProgramUniform4f(shader, location, data.x, data.y, data.z, data.w);
    };

    void updateUniform(uint32_t location, int data) {
        glProgramUniform1f(shader, location, data);
    };

    void updateUniform(uint32_t location, Vector2Int data) {
        glProgramUniform2f(shader, location, data.x, data.y);
    };

    void updateUniform(uint32_t location, Vector3Int data) {
        glProgramUniform3f(shader, location, data.x, data.y, data.z);
    };

    void updateUniform(uint32_t location, Vector4Int data) {
        glProgramUniform4f(shader, location, data.x, data.y, data.z, data.w);
    };

    void updateUniform(uint32_t location, Matrix3 data) {
        glProgramUniformMatrix3fv(shader, location, 1, GL_FALSE, &data[0][0]);
    };

    void updateUniform(uint32_t location, Matrix4 data) {
        glProgramUniformMatrix4fv(shader, location, 1, GL_FALSE, &data[0][0]);
    };

    void setTexture(uint32_t slot, Texture* texture) {
        _textures[slot] = texture;
    };

    void activateMaterial() {
        glUseProgram(shader);

        for (auto const& [slot, tex] : _textures) {
            if (tex) {
                glBindTextureUnit(slot, tex->ID);
            }
        }
    };

private:
    std::unordered_map<uint32_t, Texture*> _textures;

public:
    Material(const Material&) = delete;
    Material& operator=(const Material&) = delete;
};