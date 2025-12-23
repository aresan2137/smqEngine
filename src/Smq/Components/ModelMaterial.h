#pragma once

#include "../Smq.h"

namespace smq {
	namespace comp {
		class ModelMaterial : public Component {
		public:

			ModelMaterial(Mesh mesh, Material material, Texture textrue);
			ModelMaterial(Mesh mesh, Material material);
			ModelMaterial();

			void SetMaterial(Material material);
			void SetModel(Mesh mesh);
			void SetTexture(Texture texture);

			Material GetMaterial();
			Mesh GetModel();
			Texture GetTexture();

		private:
			Material i_material;
			Mesh i_mesh;
			Texture i_texture;
		};
	}
}