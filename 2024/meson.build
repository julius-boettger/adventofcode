project('advent-of-code', 'cpp')

executable(
    meson.project_name(),

    include_directories: 'src/include',
    sources: [
        'src/01.cpp',
    ],

    # compiler arguments (assuming gcc)
    cpp_args: [
        '-std=c++23',
        # throw errors when using compiler extensions
        '-pedantic-errors',
        # treat warnings as errors
        '-Werror',
        # enable more warnings
        '-Wall', '-Weffc++', '-Wextra', '-Wconversion', '-Wsign-conversion', '-Wshadow',
    ],
)