#version 330

in vec3 fragPos;
in vec3 fragNormal;
in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;
uniform vec3 sunLightDir;
uniform vec3 sunLightColor;
uniform vec3 ambientColor;

out vec4 finalColor;

void main()
{
    vec4 texColor = texture(texture0, fragTexCoord) * colDiffuse * fragColor;

    vec3 normal = normalize(fragNormal);
    vec3 sunLightDirN = normalize(sunLightDir);

    // Half-Lambert wrapping, remap the NdotL to 0..1 range
    float NdotL = dot(normal, -sunLightDirN);
    float halfLambert = NdotL * 0.5 + 0.5;

    // Smooth shadow and midtone bands : dark < 0.25 < mid < 0.6 < bright
    float shadow  = smoothstep(0.25, 0.55, halfLambert);
    float midtone = smoothstep(0.60, 0.75, halfLambert);

    // Blend between 3 levels: shadow(0.30) -> mid(0.65) -> bright(1.0)
    float cel_shade = mix(0.30, 0.65, shadow);
    cel_shade = mix(cel_shade, 1.0, midtone);

    vec3 ambient = ambientColor * texColor.rgb;
    vec3 diffuse = sunLightColor * texColor.rgb * cel_shade;
    vec3 result = ambient + diffuse;

    // Clamp to avoid oversaturation
    result = min(result, vec3(1.0));
    finalColor = vec4(result, texColor.a);
}
