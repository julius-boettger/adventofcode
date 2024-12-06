project('advent-of-code', 'cpp')

executable(
    meson.project_name(),

    include_directories: 'src/include',
    sources: [
        'src/util.cpp',
        'src/01/1.cpp',
        'src/01/2.cpp',
        'src/02/1.cpp',
        'src/02/2.cpp',
        'src/03/1.cpp',
        'src/03/2.cpp',
        'src/04/1.cpp',
        'src/04/2.cpp',
        'src/05/1.cpp',
        'src/05/2.cpp',
        'src/06/1.cpp',
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