#pragma once

#include "../Units.h"

namespace smq {
	namespace comp {
		class Position3D : public Component {
		public:
			Position3D() {};

			Vector3 position = { 0.0f,0.0f,0.0f };
			Vector3 rotation = { 0.0f,0.0f,0.0f };

		private:
			
		};
	}
}