#include "../Include.h"

#include <thread>
#include <atomic>
#include <mutex>

void StartObject(smq::Object* object) {

    const std::vector<smq::Component*>& components = object->GetAllComponents();
    for (int i = 0; i < components.size(); i++) {
        components[i]->Start();
    }

    const std::vector<smq::Object*>* kids = object->GetAllChildren();
    for (int i = 0; i < kids->size(); i++) {
        StartObject(kids->at(i));
    }
}

void UpdateObject(smq::Object* object, float Delta) {

    const std::vector<smq::Component*>& components = object->GetAllComponents();
    for (int i = 0; i < components.size(); i++) {
        components[i]->Update(Delta);
    }

    const std::vector<smq::Object*>* kids = object->GetAllChildren();
    for (int i = 0; i < kids->size(); i++) {
        UpdateObject(kids->at(i), Delta);
    }
}

float Time = 0.0f;
float TimePre = 0.0f;

smq::Physics phs;

namespace smq {
    void Engine::StartRuntime() {

        StartObject(currentScene->rootObject);

		phs.Start();


        while (!glfwWindowShouldClose(i_GlfwWindow)) {
            glfwSwapBuffers(i_GlfwWindow);
    
            InputUpdate(i_ImIo);
            glfwPollEvents();
            
            DrawScene();

            ImGui_ImplOpenGL3_NewFrame();
            ImGui_ImplGlfw_NewFrame();
            ImGui::NewFrame();

            Time = (float)glfwGetTime();
            float Delta = Time - TimePre;
            TimePre = Time;
            UpdateObject(currentScene->rootObject, Delta);

            glfwGetWindowSize(i_GlfwWindow, &i_window.size.x, &i_window.size.y);

            ImGui::Render();
            ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        }
        phs.Stop();
    }

    void Engine::SetScene(smq::Scene* scene) {
        currentScene = scene;
    }
    smq::Scene* Engine::GetScene() {
        return currentScene;
    }
    void Engine::SetPostProcesingMaterial(Material* material) {
        i_post = material;
    }
}

