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
        'src/06/2.cpp',
        'src/07/1.cpp',
        'src/07/2.cpp',
        'src/08/1.cpp',
        'src/08/2.cpp',
        'src/09/1.cpp',
        'src/09/2.cpp',
    ],

    # compiler arguments (assuming gcc)
    cpp_args: [
        '-std=c++23',
        # throw errors when using compiler extensions
        '-pedantic-errors',
        # treat warnings as errors
        '-Werror',
        # except some trivial warnings
        '-Wno-error=unused-parameter', '-Wno-error=unused-variable', '-Wno-error=unused-but-set-variable',
        # enable more warnings
        '-Wall', '-Weffc++', '-Wextra', '-Wconversion', '-Wsign-conversion', '-Wshadow',
    ],
)