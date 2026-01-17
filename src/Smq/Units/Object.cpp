#include "../Include.h"

namespace smq {
	Object::Object() {
		Log("Object Created Sucesfully");
	}

	Object::~Object() {
		if (i_parent) {
			i_parent->RemoveChild(this, false);
		}

		for (int i = 0; i < i_children.size(); i++) {
			if (i_children[i]) {
				i_children[i]->i_parent = nullptr;
				delete i_children[i];
			}
		}
		i_children.clear();

		for (int i = 0; i < i_components.size(); i++) {
			delete i_components[i];
		}
		i_components.clear();

		Log("Object Deleted Successfully");
	}

	void Object::AddComponent(Component* component) {
		for (int i = 0; i < i_components.size(); i++) {
			if (i_components[i] == component) {
				Warn("Object: Trying to add component to an object but component is alredy added");
				return;
			}
		}
		i_components.push_back(component);

		comp::Position3D* tmp1 = dynamic_cast<comp::Position3D*>(component);
		if (tmp1 != nullptr) {
			i_Position3D = tmp1; 
		} else {
			comp::ModelMaterial* tmp2 = dynamic_cast<comp::ModelMaterial*>(component);
			if (tmp2 != nullptr) {
				i_ModelMaterial = tmp2;
			}	
		}
	}

	void Object::RemoveComponent(Component* component, bool destroy) {
		for (int i = 0; i < i_components.size(); i++) {
			if (i_components[i] == component) {
				if (destroy) delete i_components[i];
				i_components[i] = i_components[i_components.size() - 1];
				i_components.pop_back();
				return;
			}
		}
	}

	std::vector<Component*> Object::GetAllComponents() {
		return i_components;
	}

	void Object::AddChild(Object* object) {
		for (int i = 0; i < i_children.size(); i++) {
			if (i_children[i] == object) {
				Warn("Object: Trying to add child to an object but child is alredy added");
				return;
			}
		}
		i_children.push_back(object);
		object->SetParent(this);
	}

	void Object::RemoveChild(Object* object, bool destroy) {
		for (int i = 0; i < i_children.size(); i++) {
			if (i_children[i] == object) {

				Object* toDelete = nullptr;

				if (destroy) {
					toDelete = i_children[i];
				}

				i_children[i] = i_children.back();
				i_children.pop_back();

				if (toDelete) {
					toDelete->i_parent = nullptr;
					delete toDelete;
				}
				return;
			}
		}
	}

	Object* Object::GetChild(unsigned int count) {
		return i_children[count];
	}

	std::vector<Object*> Object::GetAllChildren() {
		return i_children;
	}

	void Object::SetParent(Object* parent) {
		i_parent = parent;
	}

	Object* Object::GetParent() {
		return i_parent;
	}

	comp::Position3D* Object::GetPosition3D() {
		return i_Position3D;
	}

	comp::ModelMaterial* Object::GetModelMaterial() {
		return i_ModelMaterial;
	}
}