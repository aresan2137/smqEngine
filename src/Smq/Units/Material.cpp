#include "../Include.h"

namespace smq {

	Material::Material() {
		i_shader = 0;
		Log("Material Created Sucesfully");
	}

	Material::Material(const std::string vertexShaderFilename, const std::string fragmentShadeFilename) {
		std::string vertexShaderCode = LoadFile(vertexShaderFilename);
		std::string fragmentShaderCode = LoadFile(fragmentShadeFilename);

		i_shader = glCreateProgram();

		// vertex shader
		unsigned int vertexShader = glCreateShader(GL_VERTEX_SHADER);
		const char* vertexShaderCode_cstr = vertexShaderCode.c_str();
		glShaderSource(vertexShader, 1, &vertexShaderCode_cstr, nullptr);
		glCompileShader(vertexShader);

		// fragment shader 
		unsigned int fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
		const char* fragmentShaderCode_cstr = fragmentShaderCode.c_str();
		glShaderSource(fragmentShader, 1, &fragmentShaderCode_cstr, nullptr);
		glCompileShader(fragmentShader);

		glAttachShader(i_shader, vertexShader);
		glAttachShader(i_shader, fragmentShader);
		glLinkProgram(i_shader);
		glValidateProgram(i_shader);

		glDeleteProgram(vertexShader);
		glDeleteProgram(fragmentShader);

		glUseProgram(i_shader);
		i_mvp = glGetUniformLocation(i_shader, "u_mvp");
		i_tex = glGetUniformLocation(i_shader, "u_tex");

		Log("Material Created Sucesfully");
	}

	void Material::Delete() {
		if (i_shader != 0) {
			glDeleteProgram(i_shader);
		}
		Log("Material Deleted Sucesfully");
	}

	void Material::ActivateMaterial() {
		glUseProgram(i_shader);
	}

	void Material::UpdateAtribute(std::string name, float value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniform1f(loc, value);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::UpdateAtribute(std::string name, Vector2 value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniform2f(loc, value.x, value.y);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::UpdateAtribute(std::string name, Vector3 value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniform3f(loc, value.x, value.y, value.z);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::UpdateAtribute(std::string name, Vector4 value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniform4f(loc, value.x, value.y, value.z, value.w);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::UpdateAtribute(std::string name, int value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniform1i(loc, value);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::UpdateAtribute(std::string name, Vector2Int value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniform2i(loc, value.x, value.y);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::UpdateAtribute(std::string name, Vector3Int value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniform3i(loc, value.x, value.y, value.z);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::UpdateAtribute(std::string name, Vector4Int value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniform4i(loc, value.x, value.y, value.z, value.w);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::UpdateAtribute(std::string name, Matrix4 value) {
		ActivateMaterial();
		int loc = glGetUniformLocation(i_shader, name.c_str());
		if (loc != -1) {
			glUniformMatrix4fv(loc, 1, false, &value[0][0]);
		} else {
			smq::Warn("UpdateAtribute: update failed");
		}
	}

	void Material::SetMVP(Matrix4 value) {
		ActivateMaterial();
		if (i_mvp != -1) {
			glUniformMatrix4fv(i_mvp, 1, false, &value[0][0]);
		}
	}

	void Material::SetTexture(int slot) {
		ActivateMaterial();
		if (i_tex != -1) {
			glUniform1i(i_tex, slot);
		}
	}
}