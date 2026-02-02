#pragma once
#include "../Smq.h"

namespace smq {
	class DefultRenderer : public Renderer {
	public:
		DefultRenderer();
		~DefultRenderer();

		void Render(Scene* scene, RenderTexture* renderTexture, Matrix4 vp) override;

	private:
		SSBO i_SSBO;
	};

}