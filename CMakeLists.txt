cmake_minimum_required(VERSION 2.8)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/RustCMake/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")

add_subdirectory(wxc)

if(APPLE)
    set(PATCH_UNSAFE_RS patch -p0 < src/_unsafe.rs.patch)
    execute_process(
        COMMAND xcode-select --print-path
        OUTPUT_VARIABLE OSX_SDK
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    set(LIBCLANG_PATH ${OSX_SDK}/Toolchains/XcodeDefault.xctoolchain/usr/lib)
    set(BINDGEN_LIBPATH DYLD_FALLBACK_LIBRARY_PATH=${LIBCLANG_PATH})
    set(BINDGEN_PRELOAD)
else(APPLE)
    set(PATCH_UNSAFE_RS)
    file(GLOB LIBCLANG_PATH1 "/usr/lib/llvm*/lib")
    file(GLOB LIBCLANG_PATH2 "/usr/lib64/llvm")
    set(LIBCLANG_PATH ${LIBCLANG_PATH1}${LIBCLANG_PATH2})
    set(BINDGEN_LIBPATH LD_LIBRARY_PATH=${LIBCLANG_PATH})
    set(BINDGEN_PRELOAD LD_PRELOAD=${LIBCLANG_PATH}/libclang.so)
endif(APPLE)

set(RUSTC_FLAGS ${RUSTC_FLAGS}
    -L${LIBCLANG_PATH}
    -Lwxc
    -L.
)
set(GENSRC wxHaskell/wxc/src/include)
set(GENINPUT
    ${GENSRC}/wxc.h
)

set(GENOUTPUT
    ${CMAKE_SOURCE_DIR}/src/_unavailable.rs
    ${CMAKE_SOURCE_DIR}/src/advanced.rs
    ${CMAKE_SOURCE_DIR}/src/base.rs
    ${CMAKE_SOURCE_DIR}/src/cb.rs
    ${CMAKE_SOURCE_DIR}/src/core.rs
    ${CMAKE_SOURCE_DIR}/src/gl.rs
    ${CMAKE_SOURCE_DIR}/src/html.rs
    ${CMAKE_SOURCE_DIR}/src/media.rs
    ${CMAKE_SOURCE_DIR}/src/net.rs
    ${CMAKE_SOURCE_DIR}/src/odbc.rs
    ${CMAKE_SOURCE_DIR}/src/propertygrid.rs
    ${CMAKE_SOURCE_DIR}/src/stc.rs
    ${CMAKE_SOURCE_DIR}/src/xrc.rs
)

add_custom_command(
    OUTPUT ${CMAKE_SOURCE_DIR}/rust-bindgen/target/bindgen
    COMMAND LIBCLANG_PATH=${LIBCLANG_PATH} cargo build --verbose
    DEPENDS
        rust-bindgen/src/bgmacro.rs
        rust-bindgen/src/clang.rs
        rust-bindgen/src/clangll.rs
        rust-bindgen/src/gen.rs
        rust-bindgen/src/lib.rs
        rust-bindgen/src/parser.rs
        rust-bindgen/src/types.rs
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/rust-bindgen
)

add_custom_command(
    OUTPUT  ${CMAKE_SOURCE_DIR}/src/_unsafe.rs
    COMMAND cat src/_unsafe.rs.prefix > src/_unsafe.rs
    COMMAND ${BINDGEN_PRELOAD} ${BINDGEN_LIBPATH} LD_LIBRARY_PATH=${CMAKE_SOURCE_DIR}/rust-bindgen/target ${CMAKE_SOURCE_DIR}/rust-bindgen/target/bindgen
        -allow-bitfields
        -x c++
        `wx-config --cppflags`
        --include stdint.h
        --include time.h
        ${GENINPUT}
        >> src/_unsafe.rs
    COMMAND ${PATCH_UNSAFE_RS}
    DEPENDS ${GENINPUT} ${CMAKE_SOURCE_DIR}/rust-bindgen/target/bindgen
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

add_custom_command(
    OUTPUT ${GENOUTPUT}
    COMMAND python src/codegen.py ${GENINPUT}
    DEPENDS src/codegen.py ${GENINPUT}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

rust_crate_auto(src/lib.rs
    TARGET_NAME wxrust
    DESTINATION lib
    DEPENDS
        ${CMAKE_SOURCE_DIR}/src/_unsafe.rs
        ${GENOUTPUT}
        wxc
)

rust_doc(src/lib.rs
    TARGET_NAME doc
    DESTINATION doc
)

rust_crate_auto(tests/test.rs
    TARGET_NAME test_cmd
    DEPENDS
        tests/macros.rs
        ${wxrust_FULL_TARGET}
)

# Make Mac OS X App Bundle
if(APPLE)
    add_custom_command(
        OUTPUT Test.app
        COMMAND mkdir -p Test.app/Contents/MacOS
        COMMAND cp test  Test.app/Contents/MacOS/
        COMMAND cp ${CMAKE_SOURCE_DIR}/tests/Info.plist Test.app/Contents/
        COMMAND touch Test.app
        DEPENDS tests/Info.plist ${test_cmd_FULL_TARGET}
    )
    add_custom_target(test ALL DEPENDS Test.app)
else(APPLE)
    add_custom_target(test ALL DEPENDS ${test_cmd_FULL_TARGET})
endif(APPLE)
