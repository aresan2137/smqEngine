#include "../Include.h"

#include <thread> 
#include <chrono>

void StartObject(smq::Object* object) {

    const std::vector<smq::Component*>& components = object->GetAllComponents();
    for (int i = 0; i < components.size(); i++) {
        components[i]->Start();
    }

    const std::vector<smq::Object*>& kids = object->GetAllChildren();
    for (int i = 0; i < kids.size(); i++) {
        StartObject(kids[i]);
    }
}

void UpdateObject(smq::Object* object, float Delta) {

    const std::vector<smq::Component*>& components = object->GetAllComponents();
    for (int i = 0; i < components.size(); i++) {
        components[i]->Update(Delta);
    }

    const std::vector<smq::Object*>& kids = object->GetAllChildren();
    for (int i = 0; i < kids.size(); i++) {
        UpdateObject(kids[i], Delta);
    }
}





float Time = 0.0f;
float TimePre = 0.0f;

namespace smq {
    void StartRuntime(Scene* scene) {

        StartObject(scene->rootObject);

        InitDrawing(scene);

        while (!glfwWindowShouldClose(i_window)) {
            glfwSwapBuffers(i_window);
    
            InputUpdate();
            glfwPollEvents();
            
            DrawScene(scene);

            ImGui_ImplOpenGL3_NewFrame();
            ImGui_ImplGlfw_NewFrame();
            ImGui::NewFrame();

            Time = (float)glfwGetTime();
            float Delta = Time - TimePre;
            TimePre = Time;
            UpdateObject(scene->rootObject, Delta);

            ImGui::Render();
            ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        }
    }
}