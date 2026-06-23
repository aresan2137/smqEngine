#pragma once

#include <btBulletDynamicsCommon.h>

class PhysicsSystem;

const int COL_EVERYTHING = -1;
const int COL_NO_RAY = 1 << 8;

class PhysicsComponent : public Component {
public:
    btRigidBody* body;
    btCollisionShape* shape;
    btDiscreteDynamicsWorld* physicsWorld;

    PhysicsComponent(btCollisionShape* s, float mass, btVector3 pos, btDiscreteDynamicsWorld* world, Object* parent) {
        physicsWorld = world;
        shape = s;
        btVector3 inertia(0, 0, 0);
        if (mass > 0) shape->calculateLocalInertia(mass, inertia);

        btTransform transform;
        transform.setIdentity();
        transform.setOrigin(pos);

        btDefaultMotionState* motionState = new btDefaultMotionState(transform);

        btRigidBody::btRigidBodyConstructionInfo info(mass, motionState, shape, inertia);
        body = new btRigidBody(info);

        body->setUserPointer(parent);

        world->addRigidBody(body);
    }

    ~PhysicsComponent() {
        if (physicsWorld && body) {
            physicsWorld->removeRigidBody(body);
        }

        delete body->getMotionState();
        delete body;
        delete shape;
    }
};

class PhysicsSystem : public System {
public:
    btDiscreteDynamicsWorld* world;
    btDefaultCollisionConfiguration* collisionConfiguration;
    btCollisionDispatcher* dispatcher;
    btBroadphaseInterface* overlappingPairCache;
    btSequentialImpulseConstraintSolver* solver;

    PhysicsSystem() {
        collisionConfiguration = new btDefaultCollisionConfiguration();
        dispatcher = new btCollisionDispatcher(collisionConfiguration);
        overlappingPairCache = new btDbvtBroadphase();
        solver = new btSequentialImpulseConstraintSolver();
        world = new btDiscreteDynamicsWorld(dispatcher, overlappingPairCache, solver, collisionConfiguration);
        world->setGravity(btVector3(0, -9.8f, 0));
    }

    ~PhysicsSystem() {
        delete world;
        delete solver;
        delete overlappingPairCache;
        delete dispatcher;
        delete collisionConfiguration;
    }

    void push(Object* obj) {
        auto* phys = obj->getComponent<PhysicsComponent>();
        auto* pos = obj->getComponent<Position3D>();

        if (phys && pos && phys->body) {
            btTransform trans;
            trans.setIdentity();
            trans.setOrigin(btVector3(pos->position.x, pos->position.y, pos->position.z));

            phys->body->setWorldTransform(trans);
            if (phys->body->getMotionState()) {
                phys->body->getMotionState()->setWorldTransform(trans);
            }

            phys->body->setLinearVelocity(btVector3(0, 0, 0));
            phys->body->setAngularVelocity(btVector3(0, 0, 0));
            phys->body->clearForces();

            phys->body->activate(true);
        }
    }

    void Update(Scene* scene) override {
        world->stepSimulation(1.0f / 60.0f, 10);

        SyncPhysicsRecursive(scene->rootObject);
    }

    void SyncPhysicsRecursive(Object* obj) {
        if (obj->hasComponent<PhysicsComponent>()) {
            auto* phys = obj->getComponent<PhysicsComponent>();
            auto* posComp = obj->getComponent<Position3D>();

            if (posComp && phys->body) {
                btTransform trans;
                phys->body->getMotionState()->getWorldTransform(trans);

                posComp->position.x = trans.getOrigin().getX();
                posComp->position.y = trans.getOrigin().getY();
                posComp->position.z = trans.getOrigin().getZ();
            }
        }

        for (auto* kid : obj->kids) {
            SyncPhysicsRecursive(kid);
        }
    }
};