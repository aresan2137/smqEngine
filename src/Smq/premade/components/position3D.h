#pragma once

#include "../units.h"
#include "../runtime.h"

class Position3D : public Component {
public:
	Vector3 position = {0,0,0};
	Quaternion rotation = {1,0,0,0};
	Vector3 scale = {1,1,1};

	bool isStatic = false;

	glm::vec3 lastPosition = position;
	glm::quat lastRotation = rotation;
	glm::vec3 lastScale = scale;

	bool HasChanged() const {
		return position != lastPosition ||
			rotation != lastRotation ||
			scale != lastScale;
	}

	void MarkClean() {
		lastPosition = position;
		lastRotation = rotation;
		lastScale = scale;
	}

	Position3D() {};

	Position3D(Vector3 _position) 
		: position(_position)
	{
		MarkClean();
	};

	Position3D(Vector3 _position, Quaternion _rotation)
		: position(_position)
		, rotation(_rotation)
	{
		MarkClean();
	};

	Position3D(Vector3 _position, Quaternion _rotation, Vector3 _scale)
		: position(_position)
		, rotation(_rotation)
		, scale(_scale) 
	{
		MarkClean();
	};


	~Position3D() {};

};
