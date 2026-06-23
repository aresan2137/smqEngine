#pragma once

#include "units.h"

#include "miniz.h"

class Mesh {
public:
    enum class VertexFormat : uint8_t {
        Float3,
        Float2,
        Float4
    };

    struct VertexAttribute {
        VertexFormat format;
        uint32_t location;
    };

    // --- POPRAWKA: Te zmienne MUSZĄ być publiczne, bo reszta silnika ich używa! ---
    uint32_t vao = 0;
    uint32_t vbo = 0;
    uint32_t ibo = 0;
    uint32_t indexCount = 0;

public:
    void init(const std::vector<float>& vertices, const std::vector<uint32_t>& indices, const std::vector<VertexAttribute>& layout);

    Mesh(const std::vector<float>& vertices, const std::vector<uint32_t>& indices, const std::vector<VertexAttribute>& layout) {
        init(vertices, indices, layout);
    }

    Mesh(const std::string& path);
    Mesh(const std::vector<uint8_t>& buffer);

    ~Mesh() {
        if (vao != 0) glDeleteVertexArrays(1, &vao);
        if (vbo != 0) glDeleteBuffers(1, &vbo);
        if (ibo != 0) glDeleteBuffers(1, &ibo);
    }

    void activateMesh() {
        glBindVertexArray(vao);
    }

    uint32_t getIndexCount() {
        return indexCount;
    }

    Mesh(const Mesh&) = delete;
    Mesh& operator=(const Mesh&) = delete;
};

// ============================================================================
// 2. KROK: Struktura MeshData
// ============================================================================
struct MeshData {
    std::vector<float> vertices;
    std::vector<uint32_t> indices;
    std::vector<Mesh::VertexAttribute> layout;
};

// ============================================================================
// 3. KROK: Funkcja DecompressSMF
// ============================================================================
inline MeshData DecompressSMF(const std::vector<uint8_t>& buffer) {
    MeshData data;
    if (buffer.size() < 5) return data;

    if ((buffer[0] & 0xF0) != 0b10110000) {
        std::printf("[ERROR] Niepoprawny naglowek skompresowanego pliku SMF!\n");
        return data;
    }

    uint32_t uncompressedSize;
    std::memcpy(&uncompressedSize, &buffer[1], 4);

    std::vector<char> uncompressedBuffer(uncompressedSize);
    unsigned long destLen = uncompressedSize;

    const unsigned char* compressedDataPtr = buffer.data() + 5;
    size_t compressedSize = buffer.size() - 5;

    // --- POPRAWKA: Zmiana MZ_SUCCESS na poprawny MZ_OK ---
    int status = mz_uncompress(
        (unsigned char*)uncompressedBuffer.data(), &destLen,
        compressedDataPtr, (unsigned long)compressedSize
    );

    if (status != MZ_OK) {
        std::printf("[ERROR] Blad dekompresji miniz (kod: %d)!\n", status);
        return data;
    }

    size_t memOffset = 0;

    uint32_t vertexCount;
    uint32_t indexCount;
    std::memcpy(&vertexCount, &uncompressedBuffer[memOffset], 4); memOffset += 4;
    std::memcpy(&indexCount, &uncompressedBuffer[memOffset], 4);  memOffset += 4;

    uint8_t attrCount;
    std::memcpy(&attrCount, &uncompressedBuffer[memOffset], 1); memOffset += 1;

    for (uint8_t k = 0; k < attrCount; k++) {
        uint8_t fmt;
        std::memcpy(&fmt, &uncompressedBuffer[memOffset], 1); memOffset += 1;
        data.layout.push_back({ (Mesh::VertexFormat)fmt, (uint32_t)k });
    }

    uint32_t floatsPerVertex = 0;
    for (const auto& attr : data.layout) {
        if (attr.format == Mesh::VertexFormat::Float2) floatsPerVertex += 2;
        else if (attr.format == Mesh::VertexFormat::Float3) floatsPerVertex += 3;
        else if (attr.format == Mesh::VertexFormat::Float4) floatsPerVertex += 4;
    }

    data.vertices.resize(vertexCount * floatsPerVertex);
    std::memcpy(data.vertices.data(), &uncompressedBuffer[memOffset], data.vertices.size() * sizeof(float));
    memOffset += data.vertices.size() * sizeof(float);

    data.indices.resize(indexCount);
    std::memcpy(data.indices.data(), &uncompressedBuffer[memOffset], indexCount * sizeof(uint32_t));

    return data;
}

// ============================================================================
// 4. KROK: Implementacja metod klasy Mesh
// ============================================================================
inline void Mesh::init(const std::vector<float>& vertices, const std::vector<uint32_t>& indices, const std::vector<VertexAttribute>& layout) {
    indexCount = (uint32_t)indices.size();

    glCreateVertexArrays(1, &vao);

    glCreateBuffers(1, &vbo);
    glNamedBufferStorage(vbo, vertices.size() * sizeof(float), vertices.data(), 0);

    glCreateBuffers(1, &ibo);
    glNamedBufferStorage(ibo, indices.size() * sizeof(uint32_t), indices.data(), 0);

    uint32_t stride = 0;
    for (const auto& attr : layout) {
        if (attr.format == VertexFormat::Float3) stride += 3 * sizeof(float);
        else if (attr.format == VertexFormat::Float2) stride += 2 * sizeof(float);
        else if (attr.format == VertexFormat::Float4) stride += 4 * sizeof(float);
    }

    glVertexArrayVertexBuffer(vao, 0, vbo, 0, stride);
    glVertexArrayElementBuffer(vao, ibo);

    uintptr_t offset = 0;
    for (const auto& attr : layout) {
        glEnableVertexArrayAttrib(vao, attr.location);

        int size = (attr.format == VertexFormat::Float3) ? 3 :
            (attr.format == VertexFormat::Float2) ? 2 : 4;

        glVertexArrayAttribFormat(vao, attr.location, size, GL_FLOAT, GL_FALSE, (GLuint)offset);
        glVertexArrayAttribBinding(vao, attr.location, 0);

        offset += size * sizeof(float);
    }
}

inline Mesh::Mesh(const std::string& path) {
    std::ifstream file(path, std::ios::binary | std::ios::ate);
    if (!file.is_open()) return;

    std::streamsize size = file.tellg();
    file.seekg(0, std::ios::beg);

    std::vector<uint8_t> buffer(size);
    if (file.read((char*)buffer.data(), size)) {
        MeshData data = DecompressSMF(buffer);
        init(data.vertices, data.indices, data.layout);
    }
    file.close();
}

inline Mesh::Mesh(const std::vector<uint8_t>& buffer) {
    MeshData data = DecompressSMF(buffer);
    init(data.vertices, data.indices, data.layout);
}