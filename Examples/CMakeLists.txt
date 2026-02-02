cmake_minimum_required(VERSION 3.24)
project(Game LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB_RECURSE Game_SOURCES "src/*.cpp")

add_executable(Game ${Game_SOURCES})

set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} /NODEFAULTLIB:LIBCMT")

# Sam optymalizafion DISABLE WHEN RELEASE
set(SAM_PATH "${CMAKE_SOURCE_DIR}/src/sam.cpp") 

set_source_files_properties(${SAM_PATH} PROPERTIES COMPILE_FLAGS "/Od /Zi /GL- /bigobj")

target_compile_options(Game PRIVATE /bigobj)


# smq
add_subdirectory(../smqEngine smq_build) 

target_link_libraries(Game PRIVATE smq)

add_custom_command(
    TARGET Game POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        ${CMAKE_SOURCE_DIR}/src/resources
        $<TARGET_FILE_DIR:Game>/resources
)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/bin)