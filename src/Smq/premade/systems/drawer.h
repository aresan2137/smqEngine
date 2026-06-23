#pragma once

#include "../units.h"
#include "../runtime.h"

#include "../components/modelMaterial.h"
#include "../components/position3D.h"

class Drawer : public System {
private:

    void drawObject(Object* obj, Matrix4 VP, Matrix4 Model) {
        
        if (obj->hasComponent<Position3D>()) {
            Position3D* pos = obj->getComponent<Position3D>();
            Model = Model * glm::translate(glm::mat4(1.0f), pos->position) * glm::mat4_cast(pos->rotation) * glm::scale(glm::mat4(1.0f), pos->scale);
        }

        if (obj->hasComponent<ModelMaterial>()) {
            ModelMaterial* modmat = obj->getComponent<ModelMaterial>();
            modmat->material->updateUniform(0, VP * Model);
            modmat->material->updateUniform(1, Model);

            modmat->material->activateMaterial();
            modmat->mesh->activateMesh();

            glDrawElements(GL_TRIANGLES, modmat->mesh->getIndexCount(), GL_UNSIGNED_INT, 0);
        }

        for (size_t i = 0; i < obj->kids.size(); i++) {
            drawObject(obj->kids[i], VP, Model);
        }
    }

public:
    Drawer() {};

    void Update(Scene* scene) override {
		drawObject(scene->rootObject, scene->cameras[0].getVP(), glm::mat4(1.0f));
    }
};