#version 330 core
out vec4 FragColor;
in vec2 TexCoords;

// G buffer inputs
uniform sampler2D g_position;
uniform sampler2D g_normal;
uniform sampler2D g_albedo;
uniform sampler2D g_material;

// light inputs
uniform vec3 camera_pos;
uniform vec3 light_pos;
uniform vec3 light_color;

// Constants
const float PI = 3.14159265359;
const vec3 Fdielectric = vec3(0.04);

// Define functions
float distributionGGX(vec3 N, vec3 H, float roughness);
float geometrySchlickGGX(float NdotV, float roughness);
float geometrySmith(vec3 N, vec3 V, vec3 L, float roughness);
vec3 fresnelSchlick(float cosTheta, vec3 F0);
vec3 fresnelSchlickRoughness(float cosTheta, vec3 F0, float roughness);

void main() {
	vec3 position = texture(g_position, TexCoords).rgb;
    vec3 normal = texture(g_normal, TexCoords).rgb;

    vec3 albedo = texture(g_albedo, TexCoords).rgb;
    float metallic = texture(g_material, TexCoords).r;
    float roughness = texture(g_material, TexCoords).g;

	// Calculate direction from fragment to camera
    vec3 V = normalize(camera_pos - position);

    // Reflectance at normal incidence angle
    vec3 F0 = mix(Fdielectric, albedo, metallic);

    // Reflection vector
    vec3 R = reflect(-V, normal);

    // Light contribution
    vec3 Lo = vec3(0.0, 0.0, 0.0);
    {
        vec3 L = normalize(light_pos - position);
        vec3 H = normalize(V + L);
        float distance = length(light_pos - position);
        float attenuation = 1.0 / (distance * distance);
        vec3 radiance = light_color * attenuation;

        // BRDF
        float NDF = distributionGGX(normal, H, roughness);
        float G = geometrySmith(normal, V, L, roughness);
        vec3 F = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0);

        vec3 nominator = NDF * G * F;
        float denominator = 4 * max(dot(normal, V), 0.0) * max(dot(normal, L), 0.0);
        vec3 specular = nominator / max(denominator, 0.001);

        vec3 kS = F;
        vec3 kD = vec3(1.0) - kS;
        kD *= 1.0 - metallic;

        float NdotL = max(dot(normal, L), 0.0);

        Lo += (kD * albedo / PI + specular) * radiance * NdotL;
    }

    // Calculate ambient lighting from IBL
    vec3 F = fresnelSchlickRoughness(max(dot(normal, V), 0.0), F0, roughness);
    vec3 kD = 1.0 - F;
    kD *= 1.0 - metallic;

    vec3 ambient = albedo * kD;

    vec3 fragmentColor = ambient + Lo;

    // HDR tonemapping
    fragmentColor = fragmentColor / (fragmentColor + vec3(1.0));

    // Gamma correction
    fragmentColor = pow(fragmentColor, vec3(1.0/2.2));

    FragColor = vec4(fragmentColor, 1.0);
}

// Normal distribution function
float distributionGGX(vec3 N, vec3 H, float roughness) {
    float a = roughness*roughness;
    float a2 = a*a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return a2 / denom;
}

// Used by method below
float geometrySchlickGGX(float NdotV, float roughness) {
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float denom = NdotV * (1.0 - k) + k;

    return NdotV / denom;
}

// Normal distribution function. Describes self-shadowing of microfacets. When a surface is very rough,
// microfacets can overshadow other microfacets which reduces reflected light.
float geometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggxL = geometrySchlickGGX(NdotL, roughness);
    float ggxV = geometrySchlickGGX(NdotV, roughness);

    return ggxL * ggxV;
}

// Describes the ratio of surface reflection at different surface angles.
vec3 fresnelSchlick(float cosTheta, vec3 F0) {
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}

vec3 fresnelSchlickRoughness(float cosTheta, vec3 F0, float roughness) {
    return F0 + (max(vec3(1.0 - roughness), F0) - F0) * pow(1.0 - cosTheta, 5.0);
}