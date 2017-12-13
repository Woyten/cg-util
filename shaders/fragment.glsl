#version 120

uniform vec3 color;

varying float shade;

void main(void) {
	vec3 real_color = shade * color;
	gl_FragColor = vec4(real_color * real_color, 1.0);
}
