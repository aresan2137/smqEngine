#include "../Include.h"

namespace smq {
	Component::Component() {
	}

	Object* Component::GetParent() {
		return i_object;
	}

	void Component::SetParent(Object* object) {
		i_object = object;
	}

	void Component::Start() {

	}

	void Component::Update(float Delta) {
	
	}
}