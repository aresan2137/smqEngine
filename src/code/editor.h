#pragma once


struct EditorMesh {
    std::string name;
    std::string filepath;
    Mesh* meshData;
};

#define WIN32_LEAN_AND_MEAN
#define NOMINMAX
#include <windows.h>
#include <commdlg.h>

inline void CompressSMF(const std::string& path, const std::vector<float>& vertices, const std::vector<uint32_t>& indices, const std::vector<Mesh::VertexAttribute>& layout) {
    uint32_t vertexCount = vertices.size() / 8; // Stride = 8 (Float3 + Float2 + Float3)
    uint32_t indexCount = indices.size();
    uint8_t attrCount = (uint8_t)layout.size();

    // 1. Obliczamy rozmiar nieskompresowanych danych
    size_t uncompressedSize = 4 + 4 + 1 + attrCount + (vertices.size() * sizeof(float)) + (indices.size() * sizeof(uint32_t));
    std::vector<uint8_t> uncompressedBuffer(uncompressedSize);

    // 2. Pakujemy surowe dane do bufora (identycznie jak czyta DecompressSMF)
    size_t offset = 0;
    std::memcpy(&uncompressedBuffer[offset], &vertexCount, 4); offset += 4;
    std::memcpy(&uncompressedBuffer[offset], &indexCount, 4);  offset += 4;
    std::memcpy(&uncompressedBuffer[offset], &attrCount, 1);    offset += 1;

    for (const auto& attr : layout) {
        uint8_t fmt = (uint8_t)attr.format;
        std::memcpy(&uncompressedBuffer[offset], &fmt, 1); offset += 1;
    }
    std::memcpy(&uncompressedBuffer[offset], vertices.data(), vertices.size() * sizeof(float)); offset += vertices.size() * sizeof(float);
    std::memcpy(&uncompressedBuffer[offset], indices.data(), indices.size() * sizeof(uint32_t));

    // 3. Przygotowujemy bufor na dane skompresowane (+5 bajtów na Twój nagłówek)
    unsigned long compressedSize = mz_compressBound((unsigned long)uncompressedSize) + 5;
    std::vector<uint8_t> compressedBuffer(compressedSize);

    // Zapisujemy Twój magiczny nagłówek
    compressedBuffer[0] = 0xB0; // 0b10110000
    uint32_t uSize = (uint32_t)uncompressedSize;
    std::memcpy(&compressedBuffer[1], &uSize, 4);

    // 4. Kompresujemy biblioteką miniz
    unsigned long outLen = compressedSize - 5;
    int status = mz_compress(compressedBuffer.data() + 5, &outLen, uncompressedBuffer.data(), (unsigned long)uncompressedSize);

    if (status == MZ_OK) {
        std::ofstream file(path, std::ios::binary);
        file.write((char*)compressedBuffer.data(), outLen + 5);
        std::printf("[ENGINE] Pomyślnie skompresowano i zapisano upieczony model do: %s\n", path.c_str());
    } else {
        std::printf("[ERROR] Błąd kompresji miniz przy eksporcie!\n");
    }
}

std::string OpenFileDialog(std::string filter) {
    OPENFILENAMEA ofn;
    char szFile[260];

    ZeroMemory(&ofn, sizeof(ofn));
    ofn.lStructSize = sizeof(ofn);
    ofn.hwndOwner = NULL;

    ofn.lpstrFile = szFile;
    ofn.lpstrFile[0] = '\0';
    ofn.nMaxFile = sizeof(szFile);

    ofn.lpstrFilter = filter.c_str();
    ofn.nFilterIndex = 1;

    ofn.lpstrFileTitle = NULL;
    ofn.nMaxFileTitle = 0;
    ofn.lpstrInitialDir = NULL;

    ofn.Flags = OFN_PATHMUSTEXIST | OFN_FILEMUSTEXIST | OFN_NOCHANGEDIR;

    if (GetOpenFileNameA(&ofn) == TRUE) {
        return std::string(ofn.lpstrFile);
    }

    return "";
}

struct RTObject {
    uint32_t indexStart;
    uint32_t indexEnd;
    std::vector<float> vertices;
    std::vector<uint32_t> indices;
    int objectID;
};

struct SubRTObject {
    uint32_t indexStart;
    uint32_t indexEnd;
    glm::vec3 localAabbMin;
    glm::vec3 localAabbMax;
    std::vector<float> vertices;
    std::vector<uint32_t> indices;
    int objectID;
};
std::vector<RTObject> BakeMesh(Scene* scene, std::vector<EditorMesh> meshes) {
    std::vector<RTObject> indexs;
    uint32_t offset = 0;

    for (size_t i = 0; i < meshes.size(); i++) {
        std::ifstream file(meshes[i].filepath, std::ios::binary | std::ios::ate);
        if (!file.is_open()) {
            return {};
        }

        std::streamsize size = file.tellg();
        file.seekg(0, std::ios::beg);

        std::vector<uint8_t> buffer(size);
        if (file.read((char*)buffer.data(), size)) {
            // 1. Dekompresujemy plik do struktury pośredniej
            MeshData data = DecompressSMF(buffer);

            // 2. Tworzymy obiekt dla silnika i przypisujemy offsety
            RTObject obj = {};
            obj.indexStart = offset;
            offset += data.indices.size();
            obj.indexEnd = offset;
            obj.objectID = i;

            // 3. NAPRAWIONO: Przypisujemy gotowe, ROZPAKOWANE dane z RAM-u (używamy std::move dla prędkości)
            obj.vertices = std::move(data.vertices);
            obj.indices = std::move(data.indices);

            indexs.push_back(obj);
        }
        file.close();
    }

    return indexs;
}

struct Triangle {
    uint32_t i0, i1, i2;
    glm::vec3 centroid;
};

float CalculateSurfaceArea(const glm::vec3& min, const glm::vec3& max) {
    glm::vec3 d = max - min;
    if (d.x < 0.0f || d.y < 0.0f || d.z < 0.0f) return 0.0f;
    return 2.0f * (d.x * d.y + d.x * d.z + d.y * d.z);
}

void SplitAndSortTriangles(
    std::vector<Triangle>& triangles,
    size_t start,
    size_t end,
    const std::vector<float>& globalVertices,
    uint32_t vertexOffset,
    std::vector<SubRTObject>& outChunks,
    std::vector<uint32_t>& outIndices,
    int objectID
) {
    size_t count = end - start;
    if (count == 0) return;

    // 1. Liczymy AABB dla środków ciężkości (do cięcia) i AABB wierzchołków (dla GPU)
    glm::vec3 centroidMin(FLT_MAX), centroidMax(-FLT_MAX);
    glm::vec3 chunkMin(FLT_MAX), chunkMax(-FLT_MAX);

    for (size_t i = start; i < end; ++i) {
        centroidMin = glm::min(centroidMin, triangles[i].centroid);
        centroidMax = glm::max(centroidMax, triangles[i].centroid);

        // Pobieramy pozycje XYZ wierzchołków (stride = 8 floatów)
        for (int v = 0; v < 3; v++) {
            uint32_t idx = (v == 0) ? triangles[i].i0 : (v == 1) ? triangles[i].i1 : triangles[i].i2;

            // UWAGA: idx jest względem lokalnego pliku, ale vertices są globalne
            glm::vec3 pos(
                globalVertices[(vertexOffset + idx) * 8 + 0],
                globalVertices[(vertexOffset + idx) * 8 + 1],
                globalVertices[(vertexOffset + idx) * 8 + 2]
            );

            chunkMin = glm::min(chunkMin, pos);
            chunkMax = glm::max(chunkMax, pos);
        }
    }

    // 2. Warunek stopu: Mamy odpowiednio mało trójkątów, robimy z tego Chunk!
    // Możesz zmienić '100' na więcej lub mniej, zależnie od tego jak gęsto chcesz ciąć mapę
    if (count <= 100) {
        SubRTObject chunk;
        chunk.localAabbMin = chunkMin;
        chunk.localAabbMax = chunkMax;
        chunk.objectID = objectID;

        // Zapisujemy, gdzie ten chunk się zaczyna w NOWYM globalnym buforze indeksów
        chunk.indexStart = outIndices.size();

        // Przepisujemy posortowane indeksy trójkątów (dodając globalny offset wierzchołków)
        for (size_t i = start; i < end; ++i) {
            outIndices.push_back(vertexOffset + triangles[i].i0);
            outIndices.push_back(vertexOffset + triangles[i].i1);
            outIndices.push_back(vertexOffset + triangles[i].i2);
        }

        // Zapisujemy gdzie się kończy
        chunk.indexEnd = outIndices.size();
        outChunks.push_back(chunk);
        return;
    }

    // 3. Szukamy najdłuższej osi AABB, żeby przeciąć zbiór w najszerszym miejscu
    glm::vec3 size = centroidMax - centroidMin;
    int axis = 0; // 0 = X, 1 = Y, 2 = Z
    if (size.y > size.x && size.y > size.z) axis = 1;
    if (size.z > size.x && size.z > size.y) axis = 2;

    // 4. Sortujemy trójkąty wzdłuż tej osi
    std::sort(triangles.begin() + start, triangles.begin() + end, [axis](const Triangle& a, const Triangle& b) {
        return a.centroid[axis] < b.centroid[axis];
        });

    // 5. Tniemy zbiór trójkątów na dwie równe połówki i wywołujemy rekurencję
    size_t mid = start + count / 2;
    SplitAndSortTriangles(triangles, start, mid, globalVertices, vertexOffset, outChunks, outIndices, objectID);
    SplitAndSortTriangles(triangles, mid, end, globalVertices, vertexOffset, outChunks, outIndices, objectID);
}

class Editor : public System {
private:
    const std::string savePath = "C:\\Users\\kuba\\Desktop\\scene.json";
    const std::string smfBake = "C:\\Users\\kuba\\Desktop\\baked.smf";
    bool firstFrame = true;

    bool showAABB = false;
    uint32_t debugVAO = 0, debugVBO = 0, debugEBO = 0;
    uint32_t debugShader = 0;


    void InitDebugRenderer() {
        // Prosty shader tylko do rysowania jednokolorowych linii
        const char* vsCode = R"(
            #version 330 core
            layout (location = 0) in vec3 aPos;
            uniform mat4 MVP;
            void main() { gl_Position = MVP * vec4(aPos, 1.0); }
        )";
        const char* fsCode = R"(
            #version 330 core
            out vec4 FragColor;
            uniform vec3 color;
            void main() { FragColor = vec4(color, 1.0); }
        )";

        uint32_t vs = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vs, 1, &vsCode, NULL); glCompileShader(vs);
        uint32_t fs = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fs, 1, &fsCode, NULL); glCompileShader(fs);

        debugShader = glCreateProgram();
        glAttachShader(debugShader, vs); glAttachShader(debugShader, fs);
        glLinkProgram(debugShader);

        // Wierzchołki sześcianu (AABB od 0 do 1)
        float vertices[] = {
            0,0,0,  1,0,0,  1,1,0,  0,1,0,
            0,0,1,  1,0,1,  1,1,1,  0,1,1
        };
        // Indeksy rysujące 12 linii krawędzi sześcianu
        uint32_t indices[] = {
            0,1, 1,2, 2,3, 3,0, // Tył
            4,5, 5,6, 6,7, 7,4, // Przód
            0,4, 1,5, 2,6, 3,7  // Łączenia bocznych ścian
        };

        glGenVertexArrays(1, &debugVAO);
        glGenBuffers(1, &debugVBO);
        glGenBuffers(1, &debugEBO);

        glBindVertexArray(debugVAO);
        glBindBuffer(GL_ARRAY_BUFFER, debugVBO);
        glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, debugEBO);
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);

        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
        glEnableVertexAttribArray(0);
        glBindVertexArray(0);
    }

    void DrawAABB(Scene* scene) {
        if (!showAABB || scene->cameras.empty() || bakedObjectsInfo.empty()) return;
        if (debugVAO == 0) InitDebugRenderer(); // Inicjalizacja przy pierwszym kliknięciu

        Camera& cam = scene->cameras[0];
        glm::mat4 VP = cam.getVP();

        // Aktywujemy obraz z kamery, żeby nie rysować po interfejsie ImGui
        cam.renderTexture->activateRenderTexture();

        glUseProgram(debugShader);
        glBindVertexArray(debugVAO);

        // Ustawiamy kolor jaskrawo-zielony
        glUniform3f(glGetUniformLocation(debugShader, "color"), 0.0f, 1.0f, 0.0f);

        // Wymuszamy rysowanie samych krawędzi (wireframe) i ignorujemy głębię, 
        // żeby widzieć podział nawet przez ściany!
        glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
        glDisable(GL_DEPTH_TEST);

        for (size_t i = 0; i < scene->rootObject->kids.size(); i++) {
            Object* obj = scene->rootObject->kids[i];
            if (!obj->hasComponent<Position3D>()) continue;

            int meshIndex = static_cast<int>(obj->tag2);
            auto* transform = obj->getComponent<Position3D>();

            glm::mat4 modelMatrix = glm::translate(glm::mat4(1.0f), transform->position);
            modelMatrix = modelMatrix * glm::mat4_cast(transform->rotation);
            modelMatrix = glm::scale(modelMatrix, transform->scale);

            for (const auto& chunk : bakedObjectsInfo) {
                if (chunk.objectID == meshIndex) {
                    // Rozmiar i pozycja konkretnego pudełka
                    glm::vec3 size = chunk.localAabbMax - chunk.localAabbMin;
                    glm::mat4 localBox = glm::translate(glm::mat4(1.0f), chunk.localAabbMin) * glm::scale(glm::mat4(1.0f), size);

                    // Macierz MVP dla shadera
                    glm::mat4 MVP = VP * modelMatrix * localBox;
                    glUniformMatrix4fv(glGetUniformLocation(debugShader, "MVP"), 1, GL_FALSE, &MVP[0][0]);

                    glDrawElements(GL_LINES, 24, GL_UNSIGNED_INT, 0);
                }
            }
        }

        // Sprzątamy po sobie, żeby nie zepsuć normalnego renderowania
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
        glEnable(GL_DEPTH_TEST);
        glBindVertexArray(0);

        cam.renderTexture->unbind(cam.renderTexture->size);
    }


    void SaveScene(Scene* scene) {
        json j;

        // 1. Zapisujemy ścieżki do załadowanych Meshy
        for (const auto& m : loadedMeshes) {
            j["meshes"].push_back(m.filepath);
        }

        // 2. Zapisujemy obiekty (Transformacje + Tag2)
        for (auto* obj : scene->rootObject->kids) {
            if (obj->hasComponent<Position3D>()) {
                auto* pos = obj->getComponent<Position3D>();
                glm::vec3 euler = glm::degrees(glm::eulerAngles(pos->rotation));

                json jObj;
                jObj["meshIndex"] = static_cast<int>(obj->tag2);
                jObj["position"] = { pos->position.x, pos->position.y, pos->position.z };
                jObj["rotation"] = { euler.x, euler.y, euler.z }; // Zapisujemy Eulerem, żeby łatwiej czytać plik
                jObj["scale"] = { pos->scale.x, pos->scale.y, pos->scale.z };

                j["objects"].push_back(jObj);
            }
        }

        // 3. Zapisujemy Światła
        for (const auto& l : activeLights) {
            json jLight;
            jLight["pos"] = { l.pos.x, l.pos.y, l.pos.z };
            jLight["power"] = l.power;
            jLight["color"] = { l.color.r, l.color.g, l.color.b };
            jLight["maxReach"] = l.maxReach;

            j["lights"].push_back(jLight);
        }

        for (const auto& chunk : bakedObjectsInfo) {
            json jChunk;
            jChunk["objectID"] = chunk.objectID;
            jChunk["indexStart"] = chunk.indexStart;
            jChunk["indexEnd"] = chunk.indexEnd;
            jChunk["aabbMin"] = { chunk.localAabbMin.x, chunk.localAabbMin.y, chunk.localAabbMin.z };
            jChunk["aabbMax"] = { chunk.localAabbMax.x, chunk.localAabbMax.y, chunk.localAabbMax.z };
            j["chunks"].push_back(jChunk);
        }

        // 4. Zrzut na pulpit
        std::ofstream file(savePath);
        if (file.is_open()) {
            file << j.dump(4); // "4" daje ładne wcięcia w pliku JSON
            std::printf("[EDITOR] Zapisano scene do: %s\n", savePath.c_str());
        }
    }

    void LoadScene(Scene* scene) {
        std::ifstream file(savePath);
        if (!file.is_open()) {
            std::printf("[EDITOR] Brak pliku zapisu. Startuje z pusta scena.\n");
            return;
        }

        json j;
        file >> j;

        // 1. Czyścimy obecny stan pamięci, żeby nie robić duplikatów
        loadedMeshes.clear();
        for (auto* obj : scene->rootObject->kids) {
            delete obj; // Usuwamy stare obiekty z pamięci
        }
        scene->rootObject->kids.clear();
        activeLights.clear();

        selectedIndex = -1;
        selectedObject = nullptr;
        selectedLightIndex = -1;

        // 2. Wczytujemy Meshe
        if (j.contains("meshes")) {
            for (const auto& path : j["meshes"]) {
                EditorMesh newMesh;
                std::string p = path.get<std::string>();
                newMesh.filepath = p;
                size_t slashPos = p.find_last_of("/\\");
                newMesh.name = (slashPos != std::string::npos) ? p.substr(slashPos + 1) : p;
                newMesh.meshData = new Mesh(p);
                loadedMeshes.push_back(newMesh);
            }
        }

        // 3. Odtwarzamy Obiekty
        if (j.contains("objects")) {
            for (const auto& jObj : j["objects"]) {
                Object* obj = new Object();
                obj->tag2 = static_cast<Tag2>(jObj["meshIndex"].get<int>());

                auto* pos = new Position3D();
                pos->position = glm::vec3(jObj["position"][0], jObj["position"][1], jObj["position"][2]);

                glm::vec3 euler(jObj["rotation"][0], jObj["rotation"][1], jObj["rotation"][2]);
                pos->rotation = glm::quat(glm::radians(euler)); // Konwersja z powrotem na Quaterniona

                pos->scale = glm::vec3(jObj["scale"][0], jObj["scale"][1], jObj["scale"][2]);

                obj->addComponent(pos);

                // Przypinamy mu załadowany model
                int mIndex = static_cast<int>(obj->tag2);
                if (mIndex >= 0 && mIndex < loadedMeshes.size()) {
                    obj->addComponent(new ModelMaterial(loadedMeshes[mIndex].meshData, res.get<Material>()));
                }

                scene->rootObject->addObject(obj);
            }
        }

        // 4. Odtwarzamy Światła
        if (j.contains("lights")) {
            for (const auto& jLight : j["lights"]) {
                LightGPU l;
                l.pos = glm::vec3(jLight["pos"][0], jLight["pos"][1], jLight["pos"][2]);
                l.power = jLight["power"].get<float>();
                l.color = glm::vec3(jLight["color"][0], jLight["color"][1], jLight["color"][2]);
                l.maxReach = jLight["maxReach"].get<float>();

                // Regenerujemy AABB światła
                l.aabbMin = l.pos - glm::vec3(l.maxReach);
                l.aabbMax = l.pos + glm::vec3(l.maxReach);
                l.pad0 = 0.0f; l.pad1 = 0.0f;

                activeLights.push_back(l);
            }
        }

        std::printf("[EDITOR] Scena zaladowana pomyslnie!\n");

        // 5. NAJWAŻNIEJSZE: Tniemy i wypiekamy automatycznie po załadowaniu!
        Bake(scene);
    }

    std::vector<EditorMesh> loadedMeshes;
    int selectedIndex = -1;

    std::vector<LightGPU> activeLights;
    int selectedLightIndex = -1;
    Object* selectedObject = nullptr;
    std::vector<SubRTObject> bakedObjectsInfo;
    void UpdateSSBO(Scene* scene) {
        std::vector<MeshInfoGPU> combinedMeshesGPU;

        // 1. Lecimy po wszystkich fizycznych OBIEKTACH postawionych na mapie
        for (size_t i = 0; i < scene->rootObject->kids.size(); i++) {
            Object* obj = scene->rootObject->kids[i];

            if (!obj->hasComponent<Position3D>()) continue;

            // Twój nowy sposób na rozpoznawanie, który to Mesh:
            int meshIndex = static_cast<int>(obj->tag2);

            // Pobieramy i liczymy macierz oraz jej INWERSJĘ dla TEGO konkretnego obiektu
            auto* transform = obj->getComponent<Position3D>();
            glm::mat4 modelMatrix = glm::translate(glm::mat4(1.0f), transform->position);
            modelMatrix = modelMatrix * glm::mat4_cast(transform->rotation);
            modelMatrix = glm::scale(modelMatrix, transform->scale);
            glm::mat4 invMat = glm::inverse(modelMatrix);

            // 2. Szukamy wszystkich chunków wygenerowanych dla tego modelu
            for (const auto& chunk : bakedObjectsInfo) {
                if (chunk.objectID == meshIndex) {
                    MeshInfoGPU mInfo;
                    mInfo.indexStart = chunk.indexStart;
                    mInfo.indexEnd = chunk.indexEnd;

                    mInfo.aabbMin = chunk.localAabbMin - glm::vec3(0.01f);
                    mInfo.pad0[0] = 0.0f;
                    mInfo.pad0[1] = 0.0f;
                    mInfo.pad1 = 0.0f;
                    mInfo.aabbMax = chunk.localAabbMax + glm::vec3(0.01f);
                    mInfo.pad2 = 0.0f;

                    // Przypisujemy odwróconą macierz dla Compute Shadera
                    mInfo.invModelMatrix = invMat;

                    // Wypychamy do GPU
                    combinedMeshesGPU.push_back(mInfo);
                }
            }
        }

        // Aktualizacja SSBO - standardowo
        if (!combinedMeshesGPU.empty()) {
            UploadMeshes(combinedMeshesGPU);
        } else {
            MeshInfoGPU dummy = {};
            UploadMeshes(std::vector<MeshInfoGPU>{dummy});
        }

        if (!activeLights.empty()) {
            UploadLights(activeLights);
        }
    }


    void Bake(Scene* scene) {
        // 1. Wczytujemy surowe dane z plików
        std::vector<RTObject> objs = BakeMesh(scene, loadedMeshes);

        std::vector<float> finalVerts;
        std::vector<uint32_t> finalInds;

        bakedObjectsInfo.clear();

        // 2. Lecimy po każdym wczytanym obiekcie
        for (const auto& rto : objs) {

            // Zapisujemy offset wierzchołków dla tego konkretnego modelu
            uint32_t currentVertexOffset = finalVerts.size() / 8; // Stride to 8

            // Kopiujemy wierzchołki JEDEN RAZ. Zero duplikacji!
            finalVerts.insert(finalVerts.end(), rto.vertices.begin(), rto.vertices.end());

            // Tworzymy listę trójkątów dla tego obiektu
            std::vector<Triangle> triangles;
            for (size_t i = 0; i < rto.indices.size(); i += 3) {
                Triangle tri;
                tri.i0 = rto.indices[i + 0];
                tri.i1 = rto.indices[i + 1];
                tri.i2 = rto.indices[i + 2];

                // Obliczamy środek ciężkości (centroid) z lokalnych wierzchołków
                glm::vec3 v0(rto.vertices[tri.i0 * 8 + 0], rto.vertices[tri.i0 * 8 + 1], rto.vertices[tri.i0 * 8 + 2]);
                glm::vec3 v1(rto.vertices[tri.i1 * 8 + 0], rto.vertices[tri.i1 * 8 + 1], rto.vertices[tri.i1 * 8 + 2]);
                glm::vec3 v2(rto.vertices[tri.i2 * 8 + 0], rto.vertices[tri.i2 * 8 + 1], rto.vertices[tri.i2 * 8 + 2]);
                tri.centroid = (v0 + v1 + v2) / 3.0f;

                triangles.push_back(tri);
            }

            // 3. Sortujemy trójkąty i generujemy pocięte chunki + nowy bufor indeksów
            SplitAndSortTriangles(
                triangles,
                0,
                triangles.size(),
                finalVerts,           // Przekazujemy globalne wierzchołki do czytania pozycji
                currentVertexOffset,  // Jakie jest przesunięcie tego modelu w globalnym buforze
                bakedObjectsInfo,     // Tu wpadną wygenerowane AABB
                finalInds,            // Tu wpadną posortowane indeksy
                rto.objectID          // ID obiektu dla ECS
            );
        }

        // 4. Czyścimy stary mesh i wysyłamy nowy, zoptymalizowany na kartę
        if (baked != nullptr) {
            delete baked;
        }

        baked = new Mesh(finalVerts, finalInds, std::vector<Mesh::VertexAttribute>{
            { Mesh::VertexFormat::Float3, 0 },
            { Mesh::VertexFormat::Float2, 1 },
            { Mesh::VertexFormat::Float3, 2 }
        });

        CompressSMF(smfBake, finalVerts, finalInds, std::vector<Mesh::VertexAttribute>{
            { Mesh::VertexFormat::Float3, 0 },
            { Mesh::VertexFormat::Float2, 1 },
            { Mesh::VertexFormat::Float3, 2 }
        });

        // 5. Aktualizujemy bufory SSBO z nowymi, posortowanymi danymi
        UpdateSSBO(scene);
    }

public:
    float thinkingblody = 3000;

    Scene* savev;

    ~Editor() {
        SaveScene(savev);
    }

    void Update(Scene* scene) override {

        if (KeyPressed(Key::F2)) {
            showAABB = !showAABB;
        }

        if (showAABB) {
            DrawAABB(scene);
            log("AABB");
        }

        if (firstFrame) {
            LoadScene(scene);
            firstFrame = false;
            savev = scene;
        }

        // ==========================================
        // 1. MESH MANAGER
        // ==========================================
        ImGui::Begin("Mesh Manager");

        if (ImGui::Button("add mesh")) {
            std::string path = OpenFileDialog("smf (*.smf)\0*.smf\0all (*.*)\0*.*\0");

            if (!path.empty()) {
                EditorMesh newMesh;
                newMesh.filepath = path;

                size_t slashPos = path.find_last_of("/\\");
                newMesh.name = (slashPos != std::string::npos) ? path.substr(slashPos + 1) : path;

                newMesh.meshData = new Mesh(path);

                loadedMeshes.push_back(newMesh);
            }
        }

        ImGui::SameLine();

        if (ImGui::Button("BAKE")) {
            Bake(scene);
        }

        ImGui::Separator();

        ImGui::BeginChild("ListaMeshy", ImVec2(200, 0), true);
        for (int i = 0; i < loadedMeshes.size(); i++) {
            bool isSelected = (selectedIndex == i);

            if (ImGui::Selectable(loadedMeshes[i].name.c_str(), isSelected)) {
                selectedIndex = i;
            }
        }
        ImGui::EndChild();

        ImGui::SameLine();

        ImGui::BeginChild("Wlasciwosci", ImVec2(0, 0), true);
        if (selectedIndex >= 0 && selectedIndex < loadedMeshes.size()) {
            EditorMesh& selected = loadedMeshes[selectedIndex];

            ImGui::Text("Zaznaczony obiekt:");
            ImGui::TextColored(ImVec4(0.5f, 0.5f, 0.5f, 1.0f), "%s", selected.filepath.c_str());
            ImGui::Separator();

            ImGui::Spacing();
            if (ImGui::Button("Usun z mapy", ImVec2(100, 0))) {
                loadedMeshes.erase(loadedMeshes.begin() + selectedIndex);
                selectedIndex = -1;
            }

            if (ImGui::Button("stwurz jego objekt", ImVec2(150, 0))) {
                Object* obj = new Object();
                // --- KLUCZOWA ZMIANA TUTAJ: Przypisujemy mu ten sam ID, żeby go potem ECS zmatchował ---
                obj->tag2 = static_cast<Tag2>(selectedIndex);

                obj->addComponent(new Position3D());
                obj->addComponent(new ModelMaterial(loadedMeshes[selectedIndex].meshData, res.get<Material>()));
                scene->rootObject->addObject(obj);
            }

        } else {
            ImGui::Text("Wybierz mesh z listy po lewej...");
        }
        ImGui::EndChild();

        ImGui::End();

        // ==========================================
        // 2. OBJECT MANAGER
        // ==========================================
        ImGui::Begin("Object Manager");

        ImGui::BeginChild("ListaObjektow", ImVec2(200, 0), true);
        if (scene->rootObject != nullptr) {
            for (size_t i = 0; i < scene->rootObject->kids.size(); i++) {
                Object* child = scene->rootObject->kids[i];

                std::string objName = "Object " + std::to_string(i);
                bool isSelected = (selectedObject == child);

                if (ImGui::Selectable(objName.c_str(), isSelected)) {
                    selectedObject = child;
                }
            }
        }
        ImGui::EndChild();

        ImGui::SameLine();

        ImGui::BeginChild("Transformacje", ImVec2(0, 0), true);
        if (selectedObject != nullptr) {
            ImGui::Text("Wlasciwosci transformacji:");
            ImGui::Separator();

            if (selectedObject->hasComponent<Position3D>()) {
                auto* transform = selectedObject->getComponent<Position3D>();

                float pos[3] = { transform->position.x, transform->position.y, transform->position.z };
                if (ImGui::DragFloat3("Pozycja (m)", pos, 0.05f)) {
                    transform->position = glm::vec3(pos[0], pos[1], pos[2]);
                }

                glm::vec3 euler = glm::degrees(glm::eulerAngles(transform->rotation));
                float rot[3] = { euler.x, euler.y, euler.z };
                if (ImGui::DragFloat3("Rotacja (deg)", rot, 0.5f)) {
                    transform->rotation = glm::quat(glm::radians(glm::vec3(rot[0], rot[1], rot[2])));
                }

                float scl[3] = { transform->scale.x, transform->scale.y, transform->scale.z };
                if (ImGui::DragFloat3("Skala", scl, 0.05f)) {
                    transform->scale = glm::vec3(scl[0], scl[1], scl[2]);
                }

                ImGui::Separator();
                if (ImGui::Button("Usun obiekt z gry")) {
                    auto& kids = scene->rootObject->kids;
                    kids.erase(std::remove(kids.begin(), kids.end(), selectedObject), kids.end());
                    delete selectedObject;
                    selectedObject = nullptr;
                }
            } else {
                ImGui::TextColored(ImVec4(1, 0, 0, 1), "Ten obiekt nie ma komponentu Position3D!");
            }
        } else {
            ImGui::Text("Wybierz obiekt z listy po lewej...");
        }
        ImGui::EndChild();

        ImGui::End();

        // ==========================================
        // 3. LIGHT MANAGER
        // ==========================================
        ImGui::Begin("Light Manager");

        if (ImGui::Button("Add Light")) {
            LightGPU l;
            l.pos = glm::vec3(0.0f, 2.0f, 0.0f);
            l.power = 0.5f;

            l.color = glm::vec3(255.0f, 255.0f, 255.0f);
            l.maxReach = 15.0f;

            l.aabbMin = l.pos - glm::vec3(l.maxReach);
            l.pad0 = 0.0f;
            l.aabbMax = l.pos + glm::vec3(l.maxReach);
            l.pad1 = 0.0f;

            activeLights.push_back(l);
            selectedLightIndex = activeLights.size() - 1;
        }

        ImGui::Separator();

        ImGui::BeginChild("ListaSwiatel", ImVec2(200, 0), true);
        for (int i = 0; i < activeLights.size(); i++) {
            std::string lightName = "Swiatlo " + std::to_string(i);
            bool isSelected = (selectedLightIndex == i);

            if (ImGui::Selectable(lightName.c_str(), isSelected)) {
                selectedLightIndex = i;
            }
        }
        ImGui::EndChild();

        ImGui::SameLine();

        ImGui::BeginChild("UstawieniaSwiatla", ImVec2(0, 0), true);
        if (selectedLightIndex >= 0 && selectedLightIndex < activeLights.size()) {
            LightGPU& light = activeLights[selectedLightIndex];

            light.power *= 100;

            float lPos[3] = { light.pos.x, light.pos.y, light.pos.z };
            if (ImGui::DragFloat3("Pozycja (Light)", lPos, 0.05f)) {
                light.pos = glm::vec3(lPos[0], lPos[1], lPos[2]);
            }

            ImGui::DragFloat("Moc (Power)", &light.power, 0.1f, 0.0f, 100.0f);

            light.power /= 100;

            if (ImGui::DragFloat("Zasieg (maxReach)", &light.maxReach, 0.1f, 0.1f, 500.0f)) {
                if (light.maxReach < 0.1f) light.maxReach = 0.1f;
            }

            ImGui::InputFloat("blackbody K: ", &thinkingblody);
            ImGui::SameLine();
            if (ImGui::Button("Set")) {
                Color colore = KelvinToColor(thinkingblody);
                light.color = glm::vec3(colore.r, colore.g, colore.b);
            }

            float lCol[3] = { light.color.r / (65025 / 255), light.color.g / (65025 / 255), light.color.b / (65025 / 255) };
            if (ImGui::ColorEdit3("Kolor (RGB)", lCol)) {
                light.color = glm::vec3(lCol[0] * (65025 / 255), lCol[1] * (65025 / 255), lCol[2] * (65025 / 255));
            }

            light.aabbMin = light.pos - glm::vec3(light.maxReach);
            light.aabbMax = light.pos + glm::vec3(light.maxReach);

            ImGui::Separator();
            if (ImGui::Button("Usun Swiatlo")) {
                activeLights.erase(activeLights.begin() + selectedLightIndex);
                selectedLightIndex = -1;
            }
        } else {
            ImGui::Text("Wybierz swiatlo z listy po lewej...");
        }
        ImGui::EndChild();

        ImGui::End();

        if (!activeLights.empty()) {
            UploadLights(activeLights);
        }
        // Należałoby zawsze wywołać UpdateSSBO na wypadek gdybyś suwakiem przesuwał obiekty
        UpdateSSBO(scene);
    }
};