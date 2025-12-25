#include "../Include.h"

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

GLuint fbo;
GLuint tex;
GLuint depthTex;
smq::Mesh plane;
glm::mat4 proj;
smq::Vector2Int renderResolution;

void InitDrawing(smq::Scene& scene) {

    renderResolution = scene.camera->renderResolution;

    float aspect = (float)renderResolution.x / (float)renderResolution.y;
    proj = glm::perspective(glm::radians(scene.camera->FOV), aspect, 0.1f, 1000.0f);

    // Render Texture Init
    glGenFramebuffers(1, &fbo);
    glBindFramebuffer(GL_FRAMEBUFFER, fbo);

    
    glGenTextures(1, &tex);
    glBindTexture(GL_TEXTURE_2D, tex);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, renderResolution.x, renderResolution.y, 0, GL_RGB, GL_UNSIGNED_BYTE, nullptr);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, tex, 0);

    glGenTextures(1, &depthTex);
    glBindTexture(GL_TEXTURE_2D, depthTex);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_DEPTH_COMPONENT, renderResolution.x, renderResolution.y, 0, GL_DEPTH_COMPONENT, GL_FLOAT, nullptr);

    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    
    glFramebufferTexture2D(GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_TEXTURE_2D, depthTex, 0);

    // ScreenModel & Shaders
    plane = smq::Mesh("resources/plane.smf");

    if (!scene.camera->postProcesingMaterial.Valid()) scene.camera->postProcesingMaterial = smq::Material("resources/Shaders/no_mvp_vs.glsl", "resources/Shaders/pass_fs.glsl");
    
}

void DrawScene(smq::Scene& scene) {
    glBindFramebuffer(GL_FRAMEBUFFER, fbo);
    glViewport(0, 0, renderResolution.x, renderResolution.y);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

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

    glBindFramebuffer(GL_FRAMEBUFFER, 0);
    glViewport(0, 0, 1280, 720);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    plane.ActivateMesh();
    scene.camera->postProcesingMaterial.ActivateMaterial();

    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, tex);
    scene.camera->postProcesingMaterial.SetTexture(0);

    glActiveTexture(GL_TEXTURE1);
    glBindTexture(GL_TEXTURE_2D, depthTex);
    scene.camera->postProcesingMaterial.UpdateAtribute("u_depth", 1, false);


    glDrawElements(GL_TRIANGLES, plane.GetTriangleCount(), GL_UNSIGNED_INT, nullptr);
}