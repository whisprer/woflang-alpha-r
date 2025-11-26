# plugins/CMakeLists.txt

# Common function to define a Woflang plugin
function(woflang_add_plugin plugin_name)
    add_library(${plugin_name} SHARED ${ARGN})

    target_include_directories(${plugin_name} PRIVATE
        ${CMAKE_SOURCE_DIR}/src
        ${CMAKE_SOURCE_DIR}/src/core
        ${CMAKE_SOURCE_DIR}/plugins
    )

    # Make sure all plugin DLLs/SOs land in one directory:
    set_target_properties(${plugin_name} PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/plugins"
        LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/plugins"
        ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/plugins"
    )
endfunction()

# Simpler, explicit subdirectory layout:
add_subdirectory(arts)
add_subdirectory(constants)
add_subdirectory(crypto)
add_subdirectory(equation_solver)
add_subdirectory(games)
add_subdirectory(graph)
add_subdirectory(logic)
add_subdirectory(markov)
add_subdirectory(math)
add_subdirectory(metaphysics)
add_subdirectory(quantum)
add_subdirectory(science)
add_subdirectory(sigils)
add_subdirectory(sigil-utils)
add_subdirectory(util)
