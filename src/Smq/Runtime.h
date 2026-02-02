#include "../Smq.h"


namespace smq {

	bool IsKeyDown(Key key);
	bool IsKeyPresed(Key key);
	Vector2 GetMousePositon();
	void SetCursor(bool visible, bool LockMouse = false);

    void PlaySound3D(std::string path, smq::Vector3 pos);
    void PlaySoundNormal(std::string path);
    void PlayMusicLoop(std::string path);
	void SetMusicVolume(float volume);
}