#include "../Include.h"

namespace smq {
	SSBO::SSBO(int bindingPoint) 
		: m_BindingPoint(bindingPoint) {
		glGenBuffers(1, &m_ID);

		glBindBuffer(GL_SHADER_STORAGE_BUFFER, m_ID);
		glBindBufferBase(GL_SHADER_STORAGE_BUFFER, m_BindingPoint, m_ID);
		glBindBuffer(GL_SHADER_STORAGE_BUFFER, 0);
	}

	SSBO::~SSBO() {
		glDeleteBuffers(1, &m_ID);
	}

	void SSBO::SetData(void* data, int size) {
		glBindBuffer(GL_SHADER_STORAGE_BUFFER, m_ID);
		glBufferData(GL_SHADER_STORAGE_BUFFER, size, data, GL_DYNAMIC_DRAW);
	}

	void SSBO::ActivateSSBO() {
		glBindBuffer(GL_SHADER_STORAGE_BUFFER, m_ID);
	}

	void SSBO::SetBindingPoint(int Binding) {
		m_BindingPoint = Binding;
		glBindBufferBase(GL_SHADER_STORAGE_BUFFER, m_BindingPoint, m_ID);
	}
}