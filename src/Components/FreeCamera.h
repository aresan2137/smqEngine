#pragma once

#include "../Units.h"

namespace smq {
	namespace comp {
		class FreeCamera : public smq::Component {
		public:
			FreeCamera(smq::Camera* camera);

			void Start() override;

			void Update(float Delta) override;

		private:
			smq::Camera* i_camera;

			smq::Vector2 i_previosMousePosition;

			bool i_controling = false;

			float i_sensivity = 0.5f;
			float i_speed = 10.0f;

			float i_fps = 0.0f;
		};
	}
}

