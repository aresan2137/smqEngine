#include "Mygui.h"

MyGui::MyGui(smq::comp::Position3D* pos3d)
    : pos3db(pos3d)
{

}

void MyGui::Start() {
    
}

void MyGui::Update(float Delta) {
    ImGui::Begin("Debug Panel");

    ImGui::SliderFloat3("Position", &pos3db->position.x, -10.0f, 10.0f);
    ImGui::SliderFloat3("Rotation", &pos3db->rotation.x, -180.0f, 180.0f);
    ImGui::SliderFloat("CameraSpeed", &camspeed, 0.0f, 100.0f);
    ImGui::SliderFloat("Sensivity", &sensivty, 0.0f, 5.0f);
    ImGui::SliderFloat3("CamRotation", &camrot.x, -180.0f, 180.0f);

    ImGui::Text(("FPS: " + std::to_string(1 / Delta)).c_str());

    ImGui::End();




}