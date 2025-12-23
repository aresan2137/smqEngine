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



void DrawObject(smq::Object* object, glm::mat4 matrix) {

    smq::comp::ModelMaterial* modelmaterial = object->GetModelMaterial();;
    smq::comp::Position3D* position3d = object->GetPosition3D();

    if (position3d != nullptr) {
        matrix *= glm::translate(glm::mat4(1.0f), gtm::Mvec3(position3d->position));

        glm::vec3 eulerRad = glm::radians(glm::vec3(position3d->rotation.x, position3d->rotation.y, position3d->rotation.z));
        glm::quat rotation = glm::quat(eulerRad);
        matrix *= glm::toMat4(rotation);

    }

    if (modelmaterial != nullptr) {
        modelmaterial->GetMaterial().SetMVP(gtm::Gmat4(matrix));

        modelmaterial->GetModel().ActivateMesh();

        if (modelmaterial->GetTexture().Valid()) {
            modelmaterial->GetTexture().ActivateTexture();

            modelmaterial->GetMaterial().SetTexture(0);
        }

        glDrawElements(GL_TRIANGLES, modelmaterial->GetModel().GetTriangleCount(), GL_UNSIGNED_INT, nullptr);
    }

    const std::vector<smq::Object*>& kids = object->GetAllChildren();
    for (int i = 0; i < kids.size(); i++) {
        DrawObject(kids[i], matrix);
    }
}

float Time = 0.0f;
float TimePre = 0.0f;

namespace smq {
    void StartRuntime(Scene scene) {

        StartObject(scene.rootObject);

        glm::mat4 proj = glm::perspective(glm::radians(60.0f), 1280.0f / 720.0f, 0.1f, 1000.0f);

        while (!glfwWindowShouldClose(window)) {
            glfwSwapBuffers(window);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            
            ImGui_ImplOpenGL3_NewFrame();
            ImGui_ImplGlfw_NewFrame();
            ImGui::NewFrame();

            InputUpdate();
            glfwPollEvents();
            
            glm::vec3 glmCamPos;
            glmCamPos.x = scene.camera->position.x;
            glmCamPos.y = scene.camera->position.y;
            glmCamPos.z = scene.camera->position.z;

            float pitch = glm::radians(scene.camera->rotation.x);
            float yaw = glm::radians(scene.camera->rotation.y);

            glm::vec3 glmFront;

            glmFront.x = cos(pitch) * sin(yaw);
            glmFront.y = sin(pitch);
            glmFront.z = cos(pitch) * cos(yaw);
            glmFront = glm::normalize(glmFront);

            glm::mat4 view = glm::lookAt(glmCamPos, glmCamPos + glmFront, glm::vec3(0.0f, 1.0f, 0.0f));


            DrawObject(scene.rootObject, proj * view);


            Time = (float)glfwGetTime();
            float Delta = Time - TimePre;
            TimePre = Time;
            UpdateObject(scene.rootObject, Delta);


            ImGui::Render();
            ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        }
    }
}