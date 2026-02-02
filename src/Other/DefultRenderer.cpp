#include "DefultRenderer.h"

#include "../Include.h"

void DrawObject(smq::Object* object, glm::mat4 matrix, glm::mat3 normalMat, glm::mat4 Mod) {
    smq::comp::ModelMaterial* modelmaterial = object->GetModelMaterial();;
    smq::comp::Position3D* position3d = object->GetPosition3D();

    if (position3d != nullptr) {
        matrix = glm::translate(matrix, position3d->position);
        Mod = glm::translate(Mod, position3d->position);

        glm::vec3 eulerRad = glm::radians(position3d->rotation);
        glm::quat rotation = glm::quat(eulerRad);
        matrix *= glm::toMat4(rotation);
        Mod *= glm::toMat4(rotation);

        matrix = glm::scale(matrix, position3d->scale);
        Mod = glm::scale(Mod, position3d->scale);

        normalMat = glm::mat3(glm::toMat4(rotation));


    }

    if (modelmaterial != nullptr) {
        if (modelmaterial->GetMaterial() != nullptr && modelmaterial->GetModel() != nullptr) {
            modelmaterial->GetMaterial()->UpdateMVP(matrix);
            modelmaterial->GetMaterial()->UpdateNormalMVP(normalMat);
            modelmaterial->GetMaterial()->UpdateM(Mod);

            modelmaterial->GetMaterial()->ActivateMaterial();

            modelmaterial->GetModel()->ActivateMesh();

            glDrawElements(GL_TRIANGLES, modelmaterial->GetModel()->GetTriangleCount(), GL_UNSIGNED_INT, nullptr);
        }
    }

    const std::vector<smq::Object*>* kids = object->GetAllChildren();
    for (int i = 0; i < kids->size(); i++) {
        DrawObject(kids->at(i), matrix, normalMat, Mod);
    }
}

namespace smq {
	DefultRenderer::DefultRenderer() 
        : i_SSBO(0)
    {

	}

	DefultRenderer::~DefultRenderer() {

	}

	void DefultRenderer::Render(Scene* scene, RenderTexture* renderTexture, Matrix4 vp) {
        renderTexture->ActivateRenderTexture();
        renderTexture->Clear();

        i_SSBO.SetData(scene->lights.data(), scene->lights.size() * sizeof(Light));

        DrawObject(scene->rootObject, vp, glm::mat3(1.0f), glm::mat4(1.0f));
	}
}


