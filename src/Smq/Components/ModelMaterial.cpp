#include "../Include.h"

namespace smq {
	namespace comp {
		ModelMaterial::ModelMaterial(Mesh mesh, Material material, Texture textrue)
			: i_mesh(mesh)
			, i_material(material)
			, i_texture(textrue)
		{

		}

		ModelMaterial::ModelMaterial(Mesh mesh, Material material)
			: i_mesh(mesh)
			, i_material(material)
		{
			i_texture = Texture();
		}

		ModelMaterial::ModelMaterial() {
			i_mesh = Mesh();
			i_material = Material();
			i_texture = Texture();
		}

		void ModelMaterial::SetMaterial(Material material) {
			i_material = material;
		}

		void ModelMaterial::SetModel(Mesh mesh) {
			i_mesh = mesh;
		}

		void ModelMaterial::SetTexture(Texture texture) {
			i_texture = texture;
		}

		Material ModelMaterial::GetMaterial() {
			return i_material;
		}

		Mesh ModelMaterial::GetModel() {
			return i_mesh;
		}

		Texture ModelMaterial::GetTexture() {
			return i_texture;
		}
	}
}