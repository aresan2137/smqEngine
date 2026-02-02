#include "RigitBody.h"
#include "../Include.h"

namespace smq {
    namespace comp {
        RigidBody::RigidBody(float mass)
			: mass(mass)
        {
        
        }

        void RigidBody::Start() {
            btCollisionShape* finalShape = nullptr;

            auto sCol = GetParent()->FindComponent<SphereCollider>();
            auto bCol = GetParent()->FindComponent<BoxCollider>();

            if (sCol) {
                finalShape = sCol->shape;
            } else if (bCol) {
                finalShape = bCol->shape;
            }

            if (!finalShape) {
                return;
            }

            Vector3 startPos = GetParent()->GetPosition3D()->position;

            btTransform t;
            t.setIdentity();
            t.setOrigin(btVector3(startPos.x, startPos.y, startPos.z));

            btVector3 inertia(0, 0, 0);
            if (mass > 0.0f) finalShape->calculateLocalInertia(mass, inertia);

            btDefaultMotionState* motionState = new btDefaultMotionState(t);
            btRigidBody::btRigidBodyConstructionInfo info(mass, motionState, finalShape, inertia);
            body = new btRigidBody(info);

            if (mass == 0.0f) {
                body->setCollisionFlags(body->getCollisionFlags() | btCollisionObject::CF_STATIC_OBJECT);
            }

			phs.addRigidBody(GetParent(), body);
        }

        void RigidBody::Teleport(Vector3 newPos) {
            if (!body) return;

            btTransform trans;
            trans.setIdentity();
            trans.setOrigin(btVector3(newPos.x, newPos.y, newPos.z));

            trans.setRotation(body->getWorldTransform().getRotation());

            body->setWorldTransform(trans);

            if (body->getMotionState()) {
                body->getMotionState()->setWorldTransform(trans);
            }

            body->setLinearVelocity(btVector3(0, 0, 0));
            body->setAngularVelocity(btVector3(0, 0, 0));
            body->clearForces();
        }
    }
}