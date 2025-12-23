cmake_minimum_required(VERSION 3.24)
project(Game LANGUAGES CXX)

file(GLOB_RECURSE GAME_SOURCES "src/**.cpp")
add_executable(Game ${GAME_SOURCES})

# GLM
target_include_directories(Game PRIVATE libs/glm-1.0.2/glm)

# GLAD
add_library(glad STATIC libs/glad/src/glad.c)
target_include_directories(glad PUBLIC libs/glad/include)

# GLFW
add_subdirectory(libs/glfw-3.4)

# IMGUI
add_library(imgui
    libs/imgui-1.92.5/imgui.cpp
    libs/imgui-1.92.5/imgui_draw.cpp
    libs/imgui-1.92.5/imgui_tables.cpp
    libs/imgui-1.92.5/imgui_widgets.cpp
    libs/imgui-1.92.5/backends/imgui_impl_glfw.cpp
    libs/imgui-1.92.5/backends/imgui_impl_opengl3.cpp
)

target_include_directories(imgui PUBLIC
    libs/imgui-1.92.5
    libs/imgui-1.92.5/backends
)

target_link_libraries(imgui PUBLIC glfw)

target_link_libraries(Game PRIVATE
    glfw
    opengl32
    glad
    imgui
)

add_custom_command(
    TARGET Game POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        ${CMAKE_SOURCE_DIR}/src/resources
        $<TARGET_FILE_DIR:Game>/resources
)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}/bin/Debug)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/bin/Release)
