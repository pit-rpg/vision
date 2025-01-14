#<vertex>
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUv;
uniform mat4 matrix_model;
uniform mat4 matrix_view;
uniform mat3 matrix_normal;
out vec4 v_color;

void main() {
	v_color = vec4(normalize(matrix_normal * aNormal), 1.0);
	gl_Position = matrix_view * matrix_model * vec4(aPos.xyz, 1.0);
}




#<fragment>
in vec4 v_color;
layout (location = 0) out vec4 FragColor;

void main() {
	FragColor = v_color;
}