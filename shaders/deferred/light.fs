#version 330 core
out vec4 FragColor;
in vec2 TexCoords;

uniform sampler2D gPosition;
uniform sampler2D gNormal;
uniform sampler2D gAlbedoSpec;

uniform vec3 light_pos;
uniform vec3 view_pos;
uniform vec3 light_color;

uniform float linear;
uniform float quadratic;

void main() {
	vec3 FragPos = texture(gPosition, TexCoords).rgb;
    vec3 Normal = texture(gNormal, TexCoords).rgb;
    vec3 Albedo = texture(gAlbedoSpec, TexCoords).rgb;
	float Specular = texture(gAlbedoSpec, TexCoords).a;

	// then calculate lighting as usual
    vec3 lighting = Albedo * 0.1; // hard-coded ambient component
    vec3 viewDir = normalize(view_pos - FragPos);

	vec3 lightDir = normalize(light_pos - FragPos);
	vec3 diffuse = max(dot(Normal, lightDir), 0.0) * Albedo * light_color;

	// specular
	vec3 halfwayDir = normalize(lightDir + viewDir);  
	float spec = pow(max(dot(Normal, halfwayDir), 0.0), 16.0);
	vec3 specular = light_color * spec * Specular;

	// attenuation
	float dist = length(light_pos - FragPos);
	float attenuation = 1.0 / (1.0 + linear * dist + quadratic * dist * dist);
	diffuse *= attenuation;
	specular *= attenuation;
	lighting += diffuse + specular;        

    FragColor = vec4(lighting, 1.0);
}  