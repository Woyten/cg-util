#version 130

uniform mat4 position_transform;
uniform mat3 normal_transform;
uniform vec3 light_direction;

in vec3 vertex_position;
in vec3 vertex_normal;
in vec2 tex_coord;

out vec2 fragment_tex_coord;
out float shade;

void main() {
	gl_Position = position_transform * vec4(vertex_position, 1.0);
	fragment_tex_coord = tex_coord;
	vec3 transformed_normal = normal_transform * vertex_normal;
	float dot_product;
	if (transformed_normal == vec3(0, 0, 0)) {
		dot_product = -1;
	} else {
		dot_product = dot(normalize(light_direction),
				normalize(transformed_normal));
	}
	shade = (1 - dot_product) / 2;
}
