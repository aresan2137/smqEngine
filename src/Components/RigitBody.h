#pragma once

#include "../Units.h"
#include <btBulletDynamicsCommon.h>
#include "BoxCollider.h"
#include "SphereCollider.h"

namespace smq {
	namespace comp {
        class RigidBody : public Component {
        public:
            float mass;
            btRigidBody* body = nullptr;

            RigidBody(float mass = 0.0f);

            void Start() override;

            void Teleport(Vector3 newPos);
        };
	}
}