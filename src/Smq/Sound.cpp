#pragma once

#define MINIAUDIO_IMPLEMENTATION
#include "miniaudio.h"

#include "../Include.h"

ma_sound bgMusic;
ma_engine engine;
bool isMusicInitialized = false;

void InitSound() {
    ma_result result;
    result = ma_engine_init(NULL, &engine);
    if (result != MA_SUCCESS) smq::Error("Failed to initialize audio engine");
}

void ShutdownSound() {
    if (isMusicInitialized) {
        ma_sound_uninit(&bgMusic);
    }
    ma_engine_uninit(&engine);
}

namespace smq {

    void UpdateSoundListener(Vector3 pos, Vector3 rotation) {

    }

    void PlaySound3D(std::string path, smq::Vector3 pos) {
        smq::Warn("Play3D is not implemented yet");
    }
    void PlaySoundNormal(std::string path) {
        ma_engine_play_sound(&engine, path.c_str(), NULL);
    }

    void PlayMusicLoop(std::string path) {
        if (isMusicInitialized) {
            ma_sound_stop(&bgMusic);
            ma_sound_uninit(&bgMusic);
            isMusicInitialized = false;
        }

        ma_result result = ma_sound_init_from_file(&engine, path.c_str(), MA_SOUND_FLAG_STREAM, NULL, NULL, &bgMusic);

        if (result == MA_SUCCESS) {
            ma_sound_set_looping(&bgMusic, MA_TRUE);
            ma_sound_set_volume(&bgMusic, 0.5f);
            ma_sound_start(&bgMusic);
            isMusicInitialized = true;
        } else {
            smq::Log("AUDIO ERROR: Failed to load music: " + path);
        }
    }

    void SetMusicVolume(float volume) {
        if (isMusicInitialized) {
            ma_sound_set_volume(&bgMusic, volume);
        }
    }
}