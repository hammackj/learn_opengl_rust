# Port of Learn Opengl to Rust

This code is mostly for me to do the learnopengl.com lessons and port them to rust for my own use. Be advised stuff might not work or be wrong.

For a more complete Rust version see this repo https://github.com/srcres258/learnopengl-rust as it is recommended by LearnOpenGL.com

As I am pretty much porting the C++ code to Rust. The license is the same as the C++ code:

"All code samples, unless explicitly stated otherwise, are licensed under the terms of the CC BY-NC 4.0 license as published by Creative Commons, either version 4 of the License, or (at your option) any later version.

See https://learnopengl.com/About for more information.'


# To Build
    cargo build

# Getting Started

## Hello Window
<img src="/screenshots/01_hello_window.png" width="50%">

    cargo run --bin hello_window

## Hello Triangle
<img src="/screenshots/02_hello_triangle.png" width="50%">

    cargo run --bin hello_triangle

## Hello Triangle Indexed
<img src="/screenshots/03_hello_triangle_indexed.png" width="50%">
<img src="/screenshots/03_hello_triangle_indexed_wireframe.png" width="50%">

    cargo run --bin hello_triangle_indexed

## Hello Triangle - Exercise 1
<img src="/screenshots/04_hello_triangle_exercise_01.png" width="50%">

    cargo run --bin hello_triangle_exercise_01

## Hello Triangle - Exercise 2
<img src="/screenshots/05_hello_triangle_exercise_02.png" width="50%">

    cargo run --bin hello_triangle_exercise_02

## Hello Triangle - Exercise 3
<img src="/screenshots/06_hello_triangle_exercise_03.png" width="50%">

    cargo run --bin hello_triangle_exercise_03

## Shaders - Uniforms
<img src="/screenshots/07_shaders.png" width="50%">

    cargo run --bin shaders

## Shaders - Shader attributes
<img src="/screenshots/08_shaders_attributes.png" width="50%">

    cargo run --bin shaders    

## Shaders - Shader Object
<img src="/screenshots/09_shaders_object.png" width="50%">
This example uses a build script to copy shaders to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/09_shaders_object
    cargo run

## Shaders - Exercise 1
<img src="/screenshots/10_shaders_exercise_01.png" width="50%">
This example uses a build script to copy shaders to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/10_shaders_exercise_01
    cargo run

## Shaders - Exercise 2
<img src="/screenshots/11_shaders_exercise_02.png" width="50%">
This example uses a build script to copy shaders to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/11_shaders_exercise_02
    cargo run

## Shaders - Exercise 3
<img src="/screenshots/12_shaders_exercise_03.png" width="50%">
This example uses a build script to copy shaders to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/12_shaders_exercise_03
    cargo run    

## Textures
<img src="/screenshots/13_textures.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/13_textures
    cargo run    

## Textures Units
<img src="/screenshots/14_textures_units.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/14_textures_units
    cargo run  

## Textures Exercise 01
<img src="/screenshots/15_textures_exercise_01.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/15_textures_exercise_01
    cargo run     

## Textures Exercise 02
<img src="/screenshots/16_textures_exercise_02.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/16_textures_exercise_02
    cargo run            

## Textures Exercise 03
<img src="/screenshots/17_textures_exercise_03.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/16_textures_exercise_03
    cargo run  

## Textures Exercise 04
<img src="/screenshots/18_textures_exercise_04.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/18_textures_exercise_04
    cargo run  

## Transforms
<img src="/screenshots/19_transformations.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/19_transformations
    cargo run 

## Transforms Exercise 01
<img src="/screenshots/20_transformations_exercise_01.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/20_transformations_exercise_01
    cargo run       

## Transforms Exercise 02
<img src="/screenshots/21_transformations_exercise_02.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/21_transformations_exercise_02
    cargo run   

## Coords
<img src="/screenshots/22_coords.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/22_coords
    cargo run       

## Coords Depth
<img src="/screenshots/23_coords_depth.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/23_coords_depth
    cargo run     

## Coords More Cubes
<img src="/screenshots/24_coords_more_cubes.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/24_coords_more_cubes
    cargo run 

## Coords More Cubes Exercise 03
<img src="/screenshots/25_coords_exercise_03.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/25_coords_exercise_03
    cargo run 

## Camera
<img src="/screenshots/26_camera.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/26_camera
    cargo run 

## Camera - Walk Around
<img src="/screenshots/27_camera_walk_around.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/27_camera_walk_around
    cargo run 

## Camera - Mouse
<img src="/screenshots/28_camera_mouse.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/28_camera_mouse
    cargo run 

## Camera - Object
<img src="/screenshots/29_camera_object.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/29_camera_object
    cargo run 

## Light - Colors
<img src="/screenshots/30_light_colors.png" width="50%">
This example uses a build script to copy shaders/textures to the folder so you have to run it in place.

    cd learn_opengl/01_getting_started/30_light_colors
    cargo run 