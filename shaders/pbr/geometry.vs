#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

layout (std140) uniform Matrices
{
	mat4 proj;
	mat4 view;
};

uniform mat4 model;
uniform mat4 normal_mat;

out vec2 TexCoords;
out vec3 FragPos;
out vec3 Normal;

void main()
{
	vec4 worldPos = model * vec4(aPos, 1.0);
	FragPos = worldPos.xyz;
	Normal = mat3(normal_mat) * aNormal;

	TexCoords = aTexCoords;
	gl_Position = proj * view * worldPos;
}