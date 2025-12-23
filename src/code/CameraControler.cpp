#include "CameraControler.h"

#include "../smq/glmToMy.h"

CameraControler::CameraControler(smq::Camera* camera, MyGui* gui)
    : cam(camera) 
    , gu(gui)
{

}

void CameraControler::Start() {
    
}

void CameraControler::Update(float Delta) {

    Delta = 1.0f / 60.0f;
    smq::Vector2 now = smq::GetMousePositon();
    smq::Vector2 dif = now - last;
    last = now;

    if (smq::IsKeyPresed(smq::Key_Z)) z = !z;

    if (z) {
        float sensitivity = gu->sensivty;
        cam->rotation.y -= dif.x * sensitivity;
        cam->rotation.x -= dif.y * sensitivity;

        cam->rotation.x = glm::clamp(cam->rotation.x, -89.0f, 89.0f);
    }

    if (smq::IsKeyDown(smq::Key_N)) {
        cam->rotation = gu->camrot;
    }

    glm::vec3 forward;

    float pitchRad = glm::radians(cam->rotation.x);
    float yawRad = glm::radians(cam->rotation.y);

    forward.y = sin(pitchRad);

    float flatLength = cos(pitchRad);

    forward.x = flatLength * sin(yawRad);
    forward.z = flatLength * cos(yawRad);

    forward = glm::normalize(forward);

    glm::vec3 right = glm::normalize(glm::cross(forward, glm::vec3(0, 1, 0)));
    glm::vec3 up = glm::normalize(glm::cross(right, forward));

    float velocity = gu->camspeed * Delta;

    if (smq::IsKeyDown(smq::Key_W)) cam->position = cam->position+ gtm::Gvec3(forward * velocity);
    if (smq::IsKeyDown(smq::Key_S)) cam->position= cam->position- gtm::Gvec3(forward * velocity);

    if (smq::IsKeyDown(smq::Key_D)) cam->position= cam->position+ gtm::Gvec3(right * velocity);
    if (smq::IsKeyDown(smq::Key_A)) cam->position= cam->position- gtm::Gvec3(right * velocity);

    if (smq::IsKeyDown(smq::Key_E)) cam->position= cam->position+ gtm::Gvec3(up * velocity);
    if (smq::IsKeyDown(smq::Key_Q)) cam->position= cam->position- gtm::Gvec3(up * velocity);
}