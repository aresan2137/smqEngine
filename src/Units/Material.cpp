#include "../Include.h"

namespace smq {

	Material::Material(Shader shader, std::vector<unsigned int> maps, std::vector<GlAtribute> uniforms)
		: i_shader(shader)
		, i_maps(maps)
		, i_uniforms(uniforms)

	{
		i_mvp = glGetUniformLocation(shader, "u_mvp");
		i_normal_mvp = glGetUniformLocation(shader, "u_normal_mvp");
		i_M = glGetUniformLocation(shader, "u_model");
	}

	void Material::ActivateMaterial() {
		glUseProgram(i_shader);
		for (int i = 0; i < i_maps.size(); i++) {
			if (i_uniforms[i].type == smq::GlUniform_Float)
				glUniform1f(i_maps[i], i_uniforms[i].f);
			if (i_uniforms[i].type == smq::GlUniform_Vec2)
				glUniform2f(i_maps[i], i_uniforms[i].f2.x, i_uniforms[i].f2.y);
			if (i_uniforms[i].type == smq::GlUniform_Vec3)
				glUniform3f(i_maps[i], i_uniforms[i].f3.x, i_uniforms[i].f3.y, i_uniforms[i].f3.z);
			if (i_uniforms[i].type == smq::GlUniform_Vec4)
				glUniform4f(i_maps[i], i_uniforms[i].f4.x, i_uniforms[i].f4.y, i_uniforms[i].f4.z, i_uniforms[i].f4.w);
			if (i_uniforms[i].type == smq::GlUniform_Int)
				glUniform1i(i_maps[i], i_uniforms[i].i);
			if (i_uniforms[i].type == smq::GlUniform_IVec2)
				glUniform2i(i_maps[i], i_uniforms[i].i2.x, i_uniforms[i].i2.y);
			if (i_uniforms[i].type == smq::GlUniform_IVec3)
				glUniform3i(i_maps[i], i_uniforms[i].i3.x, i_uniforms[i].i3.y, i_uniforms[i].i3.z);
			if (i_uniforms[i].type == smq::GlUniform_IVec4)
				glUniform4i(i_maps[i], i_uniforms[i].i4.x, i_uniforms[i].i4.y, i_uniforms[i].i4.z, i_uniforms[i].i4.w);
			if (i_uniforms[i].type == smq::GlUniform_Mat3)
				glUniformMatrix3fv(i_maps[i], 1, GL_FALSE, &i_uniforms[i].m3[0][0]);
			if (i_uniforms[i].type == smq::GlUniform_Mat4)
				glUniformMatrix4fv(i_maps[i], 1, GL_FALSE, &i_uniforms[i].m4[0][0]);
		}

		for (int i = 0; i < i_textures.size(); i++) {
			i_textures[i]->ActivateTexture(i);

		}

		glUniformMatrix4fv(i_mvp, 1, GL_FALSE, &i_mvpV[0][0]);
		glUniformMatrix3fv(i_normal_mvp, 1, GL_FALSE, &i_normal_mvpV[0][0]);
		glUniformMatrix4fv(i_M, 1, GL_FALSE, &i_MV[0][0]);
	}

	void Material::UpdateUniform(MaterialUniforms uniform, float data) {
		i_uniforms[uniform].f = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, Vector2 data) {
		i_uniforms[uniform].f2 = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, Vector3 data) {
		i_uniforms[uniform].f3 = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, Vector4 data) {
		i_uniforms[uniform].f4 = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, int data) {
		i_uniforms[uniform].i = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, Vector2Int data) {
		i_uniforms[uniform].i2 = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, Vector3Int data) {
		i_uniforms[uniform].i3 = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, Vector4Int data) {
		i_uniforms[uniform].i4 = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, Matrix3 data) {
		i_uniforms[uniform].m3 = data;
	}

	void Material::UpdateUniform(MaterialUniforms uniform, Matrix4 data) {
		i_uniforms[uniform].m4 = data;
	}

	void Material::UpdateMVP(Matrix4 data) {
		i_mvpV = data;
	}

	void Material::UpdateNormalMVP(Matrix3 data) {
		i_normal_mvpV = data;
	}

	void Material::UpdateM(Matrix4 data) {
		i_MV = data;
	}

	void Material::AddTexture(Texture* texture) {
		i_textures.push_back(texture);
	}
}
