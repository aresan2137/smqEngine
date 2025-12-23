#include "../Smq.h"

namespace smq {
	struct Scene {
		Object* rootObject;
		Camera* camera;
	};

	void init(Vector2 size, std::string WindowName);
	void quit();
	void StartRuntime(Scene scene);

	bool IsKeyDown(Key key);
	bool IsKeyPresed(Key key);
	Vector2 GetMousePositon();
}