#include "../Include.h"

bool reCalculateProj = true;

namespace smq {
	Camera::Camera(RenderTexture* renderTexture, RenderType type)
		: i_renderTexture(renderTexture)
		, i_type(type)
	{
		if (i_type == RenderType_Perspective) {
			perspective.fov = 60.0f;
			perspective.nearP = 0.1f;
			perspective.farP = 1000.0f;
		} else {
			orthographic.size = 5.0f;
			orthographic.nearP = -10.0f;
			orthographic.farP = 10.0f;
		}

		ReCalculateProjection();

		Log("Camera Created Sucesfully");
	}

	void Camera::ReCalculateProjection() {
		Vector2Int Wsize = i_renderTexture->GetSize();
		float aspect = (float)Wsize.x / (float)Wsize.y;
		if (i_type == RenderType_Perspective) {
			proj = glm::perspective(glm::radians(perspective.fov), aspect, perspective.nearP, perspective.farP);
		} else {
			proj = glm::ortho(-orthographic.size * aspect * 0.5f, orthographic.size * aspect * 0.5f, -orthographic.size * 0.5f, orthographic.size * 0.5f, orthographic.nearP, orthographic.farP);
		}		
	}

	void Camera::Render() {
		float pitch = glm::radians(rotation.x);
		float yaw = glm::radians(rotation.y);

		glm::vec3 glmFront;

		glmFront.x = cos(pitch) * sin(yaw);
		glmFront.y = sin(pitch);
		glmFront.z = cos(pitch) * cos(yaw);
		glmFront = glm::normalize(glmFront);
		i_renderTexture->Render(proj * glm::lookAt(position, position + glmFront, glm::vec3(0.0f, 1.0f, 0.0f)));
	}

	RenderTexture* Camera::GetRenderTexture() {
		return i_renderTexture;
	}

	Matrix4 Camera::GetVP() {

		float pitch = glm::radians(rotation.x);
		float yaw = glm::radians(rotation.y);

		glm::vec3 glmFront;

		glmFront.x = cos(pitch) * sin(yaw);
		glmFront.y = sin(pitch);
		glmFront.z = cos(pitch) * cos(yaw);
		glmFront = glm::normalize(glmFront);
		return proj * glm::lookAt(position, position + glmFront, glm::vec3(0.0f, 1.0f, 0.0f));
	}
}