#version 330 core
out vec4 FragColor;

uniform vec3 albedo;
uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 view_pos;

in vec3 FragPos;
in vec3 Normal;

void main()
{
	// ambient
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;
  	
    // diffuse 
    vec3 norm = normalize(Normal);
    vec3 light_dir = normalize(light_pos - FragPos);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_color;

	// specular
	float specular_strength = 0.5;
	vec3 view_dir = normalize(view_pos - FragPos);
	vec3 reflect_dir = reflect(-light_dir, norm);
	float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
	vec3 specular = specular_strength * spec * light_color;

    vec3 result = (ambient + diffuse + specular) * albedo;
    FragColor = vec4(result, 1.0);
}