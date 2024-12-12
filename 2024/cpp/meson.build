project('advent-of-code', 'cpp')

# select day to run as two-digit string like '01'
day = '10'
# select puzzle of day to run, starting from 1
puzzle = '2'

executable(
    meson.project_name(),

    include_directories: 'src/include',
    sources: [
        'src/util.cpp',
        # only compile the selected source file
        'src' / day / puzzle + '.cpp',
    ],

    # compiler arguments (assuming gcc)
    cpp_args: [
        '-std=c++23',
        # set path to input for day
        '-DINPUT_PATH="input/' + day + '.txt"',
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