#version 130

uniform sampler2D tex;

in vec2 fragment_tex_coord;
in float shade;

out vec4 color;

void main(void) {
	vec4 tex_color = texture(tex, fragment_tex_coord);
	vec4 real_color = shade * tex_color;
	color = real_color * real_color;
}
