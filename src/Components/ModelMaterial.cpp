#include "../Include.h"

namespace smq {
	namespace comp {
		ModelMaterial::ModelMaterial(Mesh* mesh, Material* material)
			: i_mesh(mesh)
			, i_material(material)
		{}

		void ModelMaterial::SetMaterial(Material* material) {
			i_material = material;
		}

		void ModelMaterial::SetModel(Mesh* mesh) {
			i_mesh = mesh;
		}

		Material* ModelMaterial::GetMaterial() {
			return i_material;
		}

		Mesh* ModelMaterial::GetModel() {
			return i_mesh;
		}
	}
}