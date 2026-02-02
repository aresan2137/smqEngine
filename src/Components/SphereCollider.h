#pragma once

#include "../Units.h"

#include <BulletCollision/CollisionShapes/btSphereShape.h>

namespace smq {
	namespace comp {
        class SphereCollider : public Component {
        public:
            float radius = 1.0f;
            btSphereShape* shape = nullptr;

            void Start() override {
                shape = new btSphereShape(radius);
            }
        private:
        };
	}
}