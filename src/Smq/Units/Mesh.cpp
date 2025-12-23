#include "../Include.h"

#include <fstream>

namespace smq {
    Mesh::Mesh() {
        Log("Mesh Created Sucesfully");
    }

	Mesh::Mesh(std::string Filename) {
        std::vector<float> data;
        std::vector<unsigned int> indices;
        
        std::ifstream file(Filename, std::ios::binary);
        if (!file.is_open()) {
            Error("Mesh: loading smf file failed");
        }

        uint32_t vertexCount = 0;
        uint32_t triangleCount = 0;

        file.read(reinterpret_cast<char*>(&vertexCount), sizeof(uint32_t));
        file.read(reinterpret_cast<char*>(&triangleCount), sizeof(uint32_t));

        if (!file) {
            Error("Mesh: smf file is corupted. header is invalid");
        }

        data.resize(vertexCount * 5);
        file.read(reinterpret_cast<char*>(data.data()), vertexCount * 5 * sizeof(float));
        if (!file) {
            Error("Mesh: smf file is corupted. vertex data is invalid");
        }

        indices.resize(triangleCount * 3);
        file.read(reinterpret_cast<char*>(indices.data()), triangleCount * 3 * sizeof(uint32_t));
        if (!file) {
            Error("Mesh: smf file is corupted. index data is invalid");
        }

        file.close();

        i_triangleCount = indices.size();

        glGenVertexArrays(1, &i_vao);
        glBindVertexArray(i_vao);

        glGenBuffers(1, &i_meshID);
        glBindBuffer(GL_ARRAY_BUFFER, i_meshID);
        glBufferData(GL_ARRAY_BUFFER, sizeof(float) * data.size(), data.data(), GL_STATIC_DRAW);

        glEnableVertexAttribArray(0);
        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void*)0);

        glEnableVertexAttribArray(1);
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void*)(3 * sizeof(float)));

        glGenBuffers(1, &i_ibo);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, i_ibo);
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(unsigned int) * indices.size(), indices.data(), GL_STATIC_DRAW);
        Log("Mesh Created Sucesfully");
	}
	
    Mesh::Mesh(std::vector<float> data, std::vector<unsigned int> indices) {
        i_triangleCount = indices.size();

        glGenVertexArrays(1, &i_vao);
        glBindVertexArray(i_vao);

        glGenBuffers(1, &i_meshID);
        glBindBuffer(GL_ARRAY_BUFFER, i_meshID);
        glBufferData(GL_ARRAY_BUFFER, sizeof(float) * data.size(), data.data(), GL_STATIC_DRAW);

        glEnableVertexAttribArray(0);
        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void*)0);

        glEnableVertexAttribArray(1);
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void*)(3 * sizeof(float)));

        glGenBuffers(1, &i_ibo);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, i_ibo);
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(unsigned int) * indices.size(), indices.data(), GL_STATIC_DRAW);
        Log("Mesh Created Sucesfully");
	}

    void Mesh::Delete() {
        if (i_ibo != 0) glDeleteBuffers(1, &i_ibo);
        if (i_meshID != 0) glDeleteBuffers(1, &i_meshID);
        if (i_vao != 0) glDeleteVertexArrays(1, &i_vao);
        Log("Mesh Deleted Sucesfully");
    }

    void Mesh::ActivateMesh() {
        glBindVertexArray(i_vao);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, i_ibo);
    }

    unsigned int Mesh::GetTriangleCount() {
        return i_triangleCount;
    }
}