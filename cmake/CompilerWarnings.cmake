# cmake/CompilerWarnings.cmake
# Comprehensive compiler warning configuration

function(set_project_warnings target)
  option(WARNINGS_AS_ERRORS "Treat compiler warnings as errors" ON)

  set(MSVC_WARNINGS
    /W4           # Warning level 4
    /w14242       # Conversion warning
    /w14254       # Operator warning
    /w14263       # Member function override warning
    /w14265       # Virtual destructor warning
    /w14287       # Unsigned/negative constant mismatch
    /we4289       # Loop variable scope
    /w14296       # Expression always true/false
    /w14311       # Pointer truncation
    /w14545       # Expression before comma has no effect
    /w14546       # Function call before comma missing argument list
    /w14547       # Operator before comma has no effect
    /w14549       # Operator before comma has no effect
    /w14555       # Expression has no effect
    /w14619       # Pragma warning suppression
    /w14640       # Thread-unsafe static member initialization
    /w14826       # Conversion warning
    /w14905       # Wide string literal cast
    /w14906       # String literal cast
    /w14928       # Illegal copy-initialization
  )

  set(CLANG_WARNINGS
    -Wall
    -Wextra
    -Wpedantic
    -Wshadow
    -Wnon-virtual-dtor
    -Wold-style-cast
    -Wcast-align
    -Wunused
    -Woverloaded-virtual
    -Wconversion
    -Wsign-conversion
    -Wdouble-promotion
    -Wformat=2
    -Wimplicit-fallthrough
  )

  set(GCC_WARNINGS
    ${CLANG_WARNINGS}
    -Wmisleading-indentation
    -Wduplicated-cond
    -Wduplicated-branches
    -Wlogical-op
    -Wuseless-cast
  )

  if(WARNINGS_AS_ERRORS)
    set(CLANG_WARNINGS ${CLANG_WARNINGS} -Werror)
    set(GCC_WARNINGS ${GCC_WARNINGS} -Werror)
    set(MSVC_WARNINGS ${MSVC_WARNINGS} /WX)
  endif()

  if(MSVC)
    set(PROJECT_WARNINGS ${MSVC_WARNINGS})
  elseif(CMAKE_CXX_COMPILER_ID MATCHES ".*Clang")
    set(PROJECT_WARNINGS ${CLANG_WARNINGS})
  elseif(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    set(PROJECT_WARNINGS ${GCC_WARNINGS})
  else()
    message(WARNING "No compiler warnings set for '${CMAKE_CXX_COMPILER_ID}' compiler.")
  endif()

  target_compile_options(${target} INTERFACE ${PROJECT_WARNINGS})
endfunction()
