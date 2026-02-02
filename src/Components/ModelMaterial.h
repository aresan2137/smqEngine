#pragma once

#include "../Smq.h"

namespace smq {
	namespace comp {
		class ModelMaterial : public Component {
		public:

			ModelMaterial(Mesh* mesh, Material* material);

			void SetMaterial(Material* material);
			void SetModel(Mesh* mesh);

			Material* GetMaterial();
			Mesh* GetModel();

		private:
			Material* i_material;
			Mesh* i_mesh;
		};
	}
}