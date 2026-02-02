#include "FreeCamera.h"

#include "../Include.h"

namespace  smq {
    namespace comp {

        FreeCamera::FreeCamera(smq::Camera* camera)
            : i_camera(camera) {

        }

        void FreeCamera::Start() {

        }

        void FreeCamera::Update(float Delta) {
            smq::Vector2 mousePosition = GetMousePositon();
            smq::Vector2 mousePositionDifrance = mousePosition - i_previosMousePosition;
            i_previosMousePosition = { 1280 / 2, 720 / 2 };
            

            if (smq::IsKeyPresed(smq::Key_Z)) i_controling = !i_controling;

            SetCursor(!i_controling, i_controling);

            if (i_controling) {

                i_camera->rotation.y -= mousePositionDifrance.x * i_sensivity;
                i_camera->rotation.x -= mousePositionDifrance.y * i_sensivity;

                i_camera->rotation.x = glm::clamp(i_camera->rotation.x, -89.0f, 89.0f);
            }

            glm::vec3 forward;

            float pitchRad = glm::radians(i_camera->rotation.x);
            float yawRad = glm::radians(i_camera->rotation.y);

            forward.y = sin(pitchRad);

            float flatLength = cos(pitchRad);

            forward.x = flatLength * sin(yawRad);
            forward.z = flatLength * cos(yawRad);

            forward = glm::normalize(forward);

            glm::vec3 right = glm::normalize(glm::cross(forward, glm::vec3(0, 1, 0)));
            glm::vec3 up = glm::normalize(glm::cross(right, forward));

            float velocity = i_speed * Delta;

            if (smq::IsKeyDown(smq::Key_W)) i_camera->position = i_camera->position + forward * velocity;
            if (smq::IsKeyDown(smq::Key_S)) i_camera->position = i_camera->position - forward * velocity;
                                                                                      
            if (smq::IsKeyDown(smq::Key_D)) i_camera->position = i_camera->position + right * velocity;
            if (smq::IsKeyDown(smq::Key_A)) i_camera->position = i_camera->position - right * velocity;
                                                                                      
            if (smq::IsKeyDown(smq::Key_E)) i_camera->position = i_camera->position + up * velocity;
            if (smq::IsKeyDown(smq::Key_Q)) i_camera->position = i_camera->position - up * velocity;

            ImGui::Begin("Free Camera Setings");

            ImGui::SliderFloat("Sensivity", &i_sensivity, 0.1f, 5.0f);
            ImGui::SliderFloat("Speed", &i_speed, 0.1f, 30.0f);

            
            float fpsN = 1.0f / Delta;
            i_fps = i_fps * 0.95f + fpsN * 0.05f;

            ImGui::Text(("FPS: " + std::to_string(i_fps)).c_str());


            ImGui::End();
        }
    }
}
