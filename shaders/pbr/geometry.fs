#version 330 core
layout (location = 0) out vec3 position_out;
layout (location = 1) out vec3 normal_out;
layout (location = 2) out vec3 albedo_out;
layout (location = 3) out vec3 material_out;

//layout (location = 2) out vec4 albedo_specular_out;

in vec3 FragPos; // model translated worldpos

in vec2 TexCoords;

in vec3 Normal; // model translated normal

uniform sampler2D texture1;

// material parameters
uniform vec3  material_albedo;
uniform float material_metallic;
uniform float material_roughness;
uniform float material_ao;

void main() {
    position_out = FragPos;
	normal_out = normalize(Normal);

	// Stores material data into texture
	albedo_out = /*texture(texture1, TexCoords).rgb **/ material_albedo;
	material_out = /*texture(texture1, TexCoords).rgb **/ vec3(material_metallic, material_roughness, material_ao);
}