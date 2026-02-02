#pragma once

#include "../Units.h"

#include <BulletCollision/CollisionShapes/btBoxShape.h>

namespace smq {
	namespace comp {
        class BoxCollider : public Component {
        public:

            Vector3 size;
            btBoxShape* shape = nullptr;

            BoxCollider(Vector3 size = { 1.0f, 1.0f, 1.0f }) 
				: size(size)
            
            {
                shape = new btBoxShape(btVector3(size.x / 2.0f, size.y / 2.0f, size.z / 2.0f));
            }
            
        private:
        };
	}
}