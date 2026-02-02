#include "Include.h"
#include <chrono>

namespace smq {
    Physics::Physics() {
        i_collisionConfig = new btDefaultCollisionConfiguration();
        i_dispatcher = new btCollisionDispatcher(i_collisionConfig);
        i_broadphase = new btDbvtBroadphase();
        i_solver = new btSequentialImpulseConstraintSolver();

        i_world = new btDiscreteDynamicsWorld(i_dispatcher, i_broadphase, i_solver, i_collisionConfig);
        i_world->setGravity(btVector3(0, -9.81f, 0));
    }

    btDiscreteDynamicsWorld* Physics::GetWorld() {
        return i_world; 
    }

    std::mutex& Physics::GetMutex() {
        return i_physicsMtx; 
    }

    void Physics::Start() {
        if (i_running) return;
        i_running = true;
        i_physicsThread = std::thread(&Physics::PhysicsLoop, this);
    }

    void Physics::addRigidBody(Object* object, btRigidBody* body) {
		i_objects.push_back(object);
		i_bodys.push_back(body);
        i_world->addRigidBody(body);
    }

    void Physics::PhysicsLoop() {
        auto lastTime = std::chrono::steady_clock::now();

        while (i_running) {
            auto currentTime = std::chrono::steady_clock::now();
            float deltaTime = std::chrono::duration<float>(currentTime - lastTime).count();
            lastTime = currentTime;

            {
                std::lock_guard<std::mutex> lock(i_physicsMtx);

                if (i_world) {
                    i_world->stepSimulation(deltaTime, 10);
                }
            }

            for (int i = 0; i < i_objects.size(); i++) {
                btTransform trans;
                i_bodys[i]->getMotionState()->getWorldTransform(trans);

                btVector3 p = trans.getOrigin();
                btQuaternion r = trans.getRotation();

                i_objects[i]->GetPosition3D()->position = { p.x(), p.y(), p.z() };

                btScalar roll, pitch, yaw;
                trans.getRotation().getEulerZYX(yaw, pitch, roll);

                i_objects[i]->GetPosition3D()->rotation = { pitch, yaw, roll };
            }

            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
    }

    void Physics::Stop() {
        i_running = false;
        if (i_physicsThread.joinable()) i_physicsThread.join();

        delete i_world;
        delete i_solver;
        delete i_broadphase;
        delete i_dispatcher;
        delete i_collisionConfig;
    }
}