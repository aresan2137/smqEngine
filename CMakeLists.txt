cmake_minimum_required(VERSION 3.24)
project(SmqEngine LANGUAGES CXX)

file(GLOB_RECURSE SmqSOURCES "src/*.cpp")

add_library(smq STATIC ${SmqSOURCES})

target_include_directories(smq PUBLIC src) 

# GLM
add_library(glm INTERFACE)
target_include_directories(glm INTERFACE libs/glm-1.0.2/glm)

# GLAD
add_library(glad STATIC libs/glad/src/glad.c)
target_include_directories(glad PUBLIC libs/glad/include)

# GLFW
add_subdirectory(libs/glfw-3.4)

# IMGUI
add_library(imgui STATIC
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

# bullet Phisics
set(USE_MSVC_RUNTIME_LIBRARY_DLL ON CACHE BOOL "" FORCE)
set(INSTALL_LIBS OFF CACHE BOOL "" FORCE)
set(BUILD_UNIT_TESTS OFF CACHE BOOL "" FORCE)
set(BUILD_BULLET2_DEMOS OFF CACHE BOOL "" FORCE)
set(BUILD_EXTRAS OFF CACHE BOOL "" FORCE)
set(BUILD_CPU_DEMOS OFF CACHE BOOL "" FORCE)
set(BUILD_BULLET3 OFF CACHE BOOL "" FORCE)

add_subdirectory(libs/bullet3)

target_include_directories(smq PUBLIC libs/bullet3/src)

target_link_libraries(smq PUBLIC
    glfw
    glad
    imgui
    glm
    opengl32
    BulletDynamics
    BulletCollision
    LinearMath
)