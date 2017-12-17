#version 120

varying vec2 fragment_coord;

void main() {
	vec3 fragment_color = vec3(1.0, 1.0, 1.0);
	float factor = 1.0;
	float c_r = fragment_coord.x;
	float c_i = fragment_coord.y;
	float z_r = 0;
	float z_i = 0;
	
	for (int i = 0; i < 100; i++) {
		float new_z_r = z_r * z_r - z_i * z_i + c_r;
		float new_z_i = 2 * z_r * z_i + c_i;
		
		z_r = new_z_r;
		z_i = new_z_i;
		
		if (z_r * z_r + z_i * z_i > 4) {
			fragment_color = vec3(i/100, i/100, 1.0);
			break;
		}
	}
	gl_FragColor = vec4(fragment_color * fragment_color, 1.0);
}
