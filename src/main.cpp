#include "smq.h"

#include "code/ssfLoader.h"
#include "code/differed.h"

#include "code/json.hpp"

using json = nlohmann::json;

#include "code/editor.h"

#define MINIAUDIO_IMPLEMENTATION
#include "code/miniaudio.h"

struct SoundBuffer {
    ma_decoder decoder;
    ma_sound sound;
    bool isValid = false;
};

// Funkcja, która pożera surowe bajty z Twojego SSF i robi z nich dźwięk
SoundBuffer* LoadSoundFromMemory(ma_engine* engine, const std::vector<uint8_t>& buffer) {
    SoundBuffer* sb = new SoundBuffer();

    // Próba dekodowania i łapanie dokładnego kodu błędu
    ma_result result = ma_decoder_init_memory(buffer.data(), buffer.size(), NULL, &sb->decoder);
    if (result != MA_SUCCESS) {
        std::printf("[AUDIO ERROR] Nie udalo sie zdekodowac bufora! Kod bledu miniaudio: %d\n", result);
        delete sb;
        return nullptr;
    }

    if (ma_sound_init_from_data_source(engine, &sb->decoder, 0, NULL, &sb->sound) != MA_SUCCESS) {
        std::printf("[AUDIO ERROR] Nie udalo sie zainicjalizowac dzwieku!\n");
        ma_decoder_uninit(&sb->decoder);
        delete sb;
        return nullptr;
    }

    sb->isValid = true;
    return sb;
}

class AudioSystem : public System {
public:
    ma_engine engine;
    bool initialized = false;

    AudioSystem() {
        if (ma_engine_init(NULL, &engine) == MA_SUCCESS) {
            initialized = true;
        }
    }

    ~AudioSystem() {
        if (initialized) {
            ma_engine_uninit(&engine);
        }
    }

    void Update(Scene* scene) override {
        // Audio działa w tle, ale w przyszłości tu będziemy uaktualniać pozycję uszu gracza do dźwięku 3D!
    }
};

#pragma pack(push, 1) 
struct SSDHeader {
    char magic[4];       // 'S','S','D','M'
    uint32_t lightCount;
    uint32_t chunkCount;
    uint32_t objectCount;
};

struct SSDLight {
    glm::vec3 pos;
    float power;
    glm::vec3 color;
    float maxReach;
};

struct SSDChunk {
    int32_t objectID;
    uint32_t indexStart;
    uint32_t indexEnd;
    glm::vec3 aabbMin;
    glm::vec3 aabbMax;
};

struct SSDObject {
    int32_t meshIndex;
    glm::vec3 position;
    glm::vec3 rotationEuler;
    glm::vec3 scale;
};
#pragma pack(pop)

// ============================================================================
// FUNKCJA PARSUJĄCA SUROWE BAJTY PLIKU .SSD
// ============================================================================
void LoadSceneFromSSD(Scene* scene, const std::vector<uint8_t>& buffer, std::vector<SSDChunk>& outChunks, std::vector<LightGPU>& outLights, const std::vector<Mesh*>& originalMeshes, Material* mat) {
    if (buffer.size() < sizeof(SSDHeader)) {
        std::printf("[ERROR] Plik .ssd jest uszkodzony lub za maly!\n");
        return;
    }

    size_t memOffset = 0;
    SSDHeader header;
    std::memcpy(&header, &buffer[memOffset], sizeof(SSDHeader));
    memOffset += sizeof(SSDHeader);

    if (header.magic[0] != 'S' || header.magic[1] != 'S' || header.magic[2] != 'D' || header.magic[3] != 'M') {
        std::printf("[ERROR] Blad krytyczny: Magiczne bajty pliku sceny sie nie zgadzaja!\n");
        return;
    }

    // [.. Dekodowanie świateł i chunków zostaje bez zmian ..]
    for (uint32_t i = 0; i < header.lightCount; i++) {
        SSDLight l;
        std::memcpy(&l, &buffer[memOffset], sizeof(SSDLight)); memOffset += sizeof(SSDLight);
        LightGPU lgpu; lgpu.pos = l.pos; lgpu.power = l.power; lgpu.color = l.color; lgpu.maxReach = l.maxReach;
        lgpu.aabbMin = l.pos - glm::vec3(l.maxReach); lgpu.aabbMax = l.pos + glm::vec3(l.maxReach);
        lgpu.pad0 = 0.0f; lgpu.pad1 = 0.0f; outLights.push_back(lgpu);
    }

    outChunks.resize(header.chunkCount);
    if (header.chunkCount > 0) {
        std::memcpy(outChunks.data(), &buffer[memOffset], header.chunkCount * sizeof(SSDChunk));
        memOffset += header.chunkCount * sizeof(SSDChunk);
    }

    // 3. Odbudowujemy drzewo ECS gry i DAJEMY IM KOMPONENTY MATERIALU!
    for (uint32_t i = 0; i < header.objectCount; i++) {
        SSDObject o;
        std::memcpy(&o, &buffer[memOffset], sizeof(SSDObject));
        memOffset += sizeof(SSDObject);

        Object* obj = new Object();
        obj->tag2 = static_cast<Tag2>(o.meshIndex);

        auto* pos = new Position3D();
        pos->position = o.position;
        pos->rotation = glm::quat(glm::radians(o.rotationEuler));
        pos->scale = o.scale;
        obj->addComponent(pos);

        // NOWOŚĆ: Skoro nie ruszamy DifferedDrawer, symulujemy zachowanie edytora!
        int meshId = static_cast<int>(o.meshIndex);
        if (meshId >= 0 && meshId < originalMeshes.size()) {
            // Przypisujemy konkretny mesh (np. suzanne) do obiektu na scenie
            obj->addComponent(new ModelMaterial(originalMeshes[meshId], mat));
        }

        scene->rootObject->addObject(obj);
    }
}

// ============================================================================
// BUDOWANIE STRUKTURY SSBO DLA COMPUTE SHADERA W TRYBIE GRY
// ============================================================================
void UpdateRuntimeSSBO(Scene* scene, const std::vector<SSDChunk>& chunks) {
    std::vector<MeshInfoGPU> combinedMeshesGPU;

    for (size_t i = 0; i < scene->rootObject->kids.size(); i++) {
        Object* obj = scene->rootObject->kids[i];
        if (!obj->hasComponent<Position3D>()) continue;

        int meshIndex = static_cast<int>(obj->tag2);
        auto* transform = obj->getComponent<Position3D>();

        // Generujemy macierz świata oraz jej inwersję w locie dla Compute Shadera
        glm::mat4 modelMatrix = glm::translate(glm::mat4(1.0f), transform->position);
        modelMatrix = modelMatrix * glm::mat4_cast(transform->rotation);
        modelMatrix = glm::scale(modelMatrix, transform->scale);
        glm::mat4 invMat = glm::inverse(modelMatrix);

        // Mapujemy pre-bakowane kostki AABB z pliku na konkretne instancje obiektów w grze
        for (const auto& chunk : chunks) {
            if (chunk.objectID == meshIndex) {
                MeshInfoGPU mInfo;
                mInfo.indexStart = chunk.indexStart;
                mInfo.indexEnd = chunk.indexEnd;
                mInfo.aabbMin = chunk.aabbMin - glm::vec3(0.01f);
                mInfo.pad0[0] = 0.0f; mInfo.pad0[1] = 0.0f; mInfo.pad1 = 0.0f;
                mInfo.aabbMax = chunk.aabbMax + glm::vec3(0.01f);
                mInfo.pad2 = 0.0f;
                mInfo.invModelMatrix = invMat;
                combinedMeshesGPU.push_back(mInfo);
            }
        }
    }

    if (!combinedMeshesGPU.empty()) {
        UploadMeshes(combinedMeshesGPU);
    } else {
        MeshInfoGPU dummy = {};
        UploadMeshes(std::vector<MeshInfoGPU>{dummy});
    }
}

// ============================================================================
// GŁÓWNY PUNKT STARTOWY CZZYSTEJ GRY (Editorless Runtime)
// ============================================================================

class RuntimeSystem : public System {
private:
    std::vector<SSDChunk> chunks;
    std::vector<LightGPU> lights;

public:

    GLFWwindow* window;
    RuntimeSystem(const std::vector<SSDChunk>& sceneChunks, const std::vector<LightGPU>& sceneLights)
        : chunks(sceneChunks), lights(sceneLights) {
        window = res.get<Window>()->window;
    }

    bool fullscreen = false;
    int windowedX = 100, windowedY = 100;
    int windowedWidth = 800, windowedHeight = 600;
    

    void Update(Scene* scene) override {
        if (KeyPressed(Key::F11)) {
            fullscreen = !fullscreen;

            if (fullscreen) {
                // 1. ZAPAMIĘTUJEMY POZYCJĘ I ROZMIAR OKNA
                glfwGetWindowPos(window, &windowedX, &windowedY);
                glfwGetWindowSize(window, &windowedWidth, &windowedHeight);

                // 2. POBIERAMY GŁÓWNY MONITOR I JEGO TRYB WIDEO
                GLFWmonitor* primaryMonitor = glfwGetPrimaryMonitor();
                const GLFWvidmode* mode = glfwGetVideoMode(primaryMonitor);

                // 3. ODPALAMY FULLSCREEN
                // Podajemy: okno, monitor, pozycję (0,0), rozdzielczość monitora i odświeżanie (Hz)
                glfwSetWindowMonitor(window, primaryMonitor, 0, 0, mode->width, mode->height, mode->refreshRate);

            } else {
                // 4. POWRÓT DO TRYBU OKIENKOWEGO
                // Przywracamy zapamiętane wcześniej współrzędne i rozmiar. Monitor ustawiamy na NULL.
                // Ostatni parametr (refresh rate) dla trybu okienkowego jest ignorowany (GLFW_DONT_CARE).
                glfwSetWindowMonitor(window, NULL, windowedX, windowedY, windowedWidth, windowedHeight, GLFW_DONT_CARE);

            }
        }
        // 1. Każda klatka odświeża i binduje światła na GPU
        if (!lights.empty()) {
            UploadLights(lights);
        }

        // 2. Każda klatka przelicza macierze świata i inwersje (aktualizuje inversTransformation)
        std::vector<MeshInfoGPU> combinedMeshesGPU;

        for (size_t i = 0; i < scene->rootObject->kids.size(); i++) {
            Object* obj = scene->rootObject->kids[i];
            if (!obj->hasComponent<Position3D>()) continue;

            int meshIndex = static_cast<int>(obj->tag2);
            auto* transform = obj->getComponent<Position3D>();

            // Liczymy transformacje dokładnie tak jak edytor
            glm::mat4 modelMatrix = glm::translate(glm::mat4(1.0f), transform->position);
            modelMatrix = modelMatrix * glm::mat4_cast(transform->rotation);
            modelMatrix = glm::scale(modelMatrix, transform->scale);
            glm::mat4 invMat = glm::inverse(modelMatrix);

            for (const auto& chunk : chunks) {
                if (chunk.objectID == meshIndex) {
                    MeshInfoGPU mInfo;
                    mInfo.indexStart = chunk.indexStart;
                    mInfo.indexEnd = chunk.indexEnd;
                    mInfo.aabbMin = chunk.aabbMin - glm::vec3(0.01f);
                    mInfo.pad0[0] = 0.0f; mInfo.pad0[1] = 0.0f; mInfo.pad1 = 0.0f;
                    mInfo.aabbMax = chunk.aabbMax + glm::vec3(0.01f);
                    mInfo.pad2 = 0.0f;
                    mInfo.invModelMatrix = invMat; // Aktualna inwersja leci do Compute Shadera
                    combinedMeshesGPU.push_back(mInfo);
                }
            }
        }

        // 3. Wypychamy zaktualizowaną strukturę akceleracji na GPU
        if (!combinedMeshesGPU.empty()) {
            UploadMeshes(combinedMeshesGPU);
        } else {
            MeshInfoGPU dummy = {};
            UploadMeshes(std::vector<MeshInfoGPU>{dummy});
        }
    }
};


int main() {
    Window window({ 1280, 720 });
    res.insert(&window);

    assets = LoadSSF("assets/game.ssf");

    Shader shader = LoadShader(*gssf<std::string>(ssf::shaders_basic_vs), *gssf<std::string>(ssf::shaders_basic_fs));
    Material material(shader);
    material.setTexture(2, gssf<Texture>(ssf::rok));
    res.insert(&material);

    Scene scene;
    res.insert(&scene);

    scene.addSystem(new GlManager());
    scene.addSystem(new DifferedDrawer(LoadComputeShader(*gssf<std::string>(ssf::shaders_compute))));

    Object* rootObj = new Object();
    scene.rootObject = rootObj;

    if (false) { // editor
        baked = new Mesh(std::vector<float>{}, std::vector<uint32_t>{}, std::vector<Mesh::VertexAttribute>{{Mesh::VertexFormat::Float3, 0}, { Mesh::VertexFormat::Float2, 1 }, { Mesh::VertexFormat::Float3, 2 }});
        scene.addSystem(new Editor());
    } else {
        baked = gssf<Mesh>(ssf::baked);

        std::vector<Mesh*> runtimeMeshes;
        // Przykład (podmień na swoje enumy z ssf):
        runtimeMeshes.push_back(gssf<Mesh>(ssf::walo));
        runtimeMeshes.push_back(gssf<Mesh>(ssf::Suzanne));
        runtimeMeshes.push_back(gssf<Mesh>(ssf::Plane));
        runtimeMeshes.push_back(gssf<Mesh>(ssf::Wall));

        std::vector<SSDChunk> runtimeChunks;
        std::vector<LightGPU> runtimeLights;


        std::vector<uint8_t>* sceneBuffer = gssf<std::vector<uint8_t>>(ssf::scene);
        if (sceneBuffer != nullptr) {
            // Przekazujemy tablicę modeli i materiał prosto do parsera
            LoadSceneFromSSD(&scene, *sceneBuffer, runtimeChunks, runtimeLights, runtimeMeshes, res.get<Material>());
        }
        scene.addSystem(new RuntimeSystem(runtimeChunks, runtimeLights));
    }

    scene.addSystem(new FreeCamera());

    AudioSystem* audioSys = new AudioSystem();
    scene.addSystem(audioSys);

    std::vector<uint8_t>* trackBuffer = gssf<std::vector<uint8_t>>(ssf::music); // Twoj enum
    SoundBuffer* ambientMusic = nullptr;

    if (trackBuffer != nullptr && audioSys->initialized) {
        // SPRAWDZAMY CZY BAJTY DOTARŁY!

        ambientMusic = LoadSoundFromMemory(&audioSys->engine, *trackBuffer);

        if (ambientMusic && ambientMusic->isValid) {
            ma_sound_set_looping(&ambientMusic->sound, MA_TRUE);
            ma_sound_set_volume(&ambientMusic->sound, 0.4f);
            ma_sound_start(&ambientMusic->sound);
        }
    }

    ma_sound_set_volume(&ambientMusic->sound, 0.8f); // 1.0 to 100% głośności!

    //RenderTexture* renderTexture = new RenderTexture({ 16 * 24, 9 * 24 }, false, true);
    RenderTexture* renderTexture = new RenderTexture({ 1280, 720 }, false, true);
    Camera cam(renderTexture);
    cam.position = { 0, 0, 0 };
    scene.cameras.push_back(cam);
    scene.output = renderTexture;

    Runtime runtime(window);

    runtime.AddScene(&scene);

    runtime.StartRuntime();

    for (size_t i = 0; i < scene.systems.size(); i++) {
        delete scene.systems[i];
    }

    if (ambientMusic && ambientMusic->isValid) {
        ma_sound_uninit(&ambientMusic->sound);
        ma_decoder_uninit(&ambientMusic->decoder);
        delete ambientMusic;
    }
}