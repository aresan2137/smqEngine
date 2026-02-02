#pragma once

#include "../Units.h"

namespace smq {
	namespace comp {
		class Position3D : public Component {
		public:
			Position3D(Vector3 position = { 0.0f,0.0f,0.0f }, Vector3 rotation = { 0.0f,0.0f,0.0f }, Vector3 scale = { 1.0f,1.0f,1.0f })
				: position(position)
				, rotation(rotation)
				, scale(scale)
			{}

			Vector3 position;
			Vector3 rotation;
			Vector3 scale;

		private:
			
		};
	}
}