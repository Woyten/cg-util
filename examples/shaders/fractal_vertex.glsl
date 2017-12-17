#version 120

uniform mat4 transform;

attribute vec2 coord;

varying vec2 fragment_coord;

void main() {
    gl_Position = transform * vec4( coord, 0.0, 1.0);
    fragment_coord = coord;
}
