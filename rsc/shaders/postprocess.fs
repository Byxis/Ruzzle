#version 330

in vec2 fragTexCoord;

uniform sampler2D texture0;
uniform vec2 resolution;

uniform vec3 bgTop;
uniform vec3 bgBottom;

out vec4 finalColor;

/// Apply a vignette effect to the color, using the UV coordinate to determine distance from the center.
/// Warn: unused
vec3 applyVignette(vec3 color, vec2 uv) {
    vec2 centered = uv - 0.5;
    float dist = length(centered);
    float vignette = smoothstep(0.75, 0.35, dist);
    return color * vignette;
}

/// Apply a warm color grading effect to the color, with shadow and highlight tints.
/// Warn: unused
vec3 applyWarmGrading(vec3 color) {
    vec3 shadowTint = vec3(0.15, 0.1, 0.05);
    vec3 highlightTint = vec3(1.05, 0.95, 0.85);
    vec3 graded = color * highlightTint + shadowTint * (1.0 - color);

    float luminance = dot(graded, vec3(0.299, 0.587, 0.114));
    graded = mix(vec3(luminance), graded, 1.15);

    return clamp(graded, 0.0, 1.0);
}

/// Apply a bloom effect to the color, using a simple 5x5 box blur approximation.
vec3 applyBloom(vec2 uv) {
    vec2 texelSize = 1.0 / resolution;

    float bloomRadius = 3.0;
    float bloomStrength = 0.45;
    float brightnessThreshold = 0.55;

    vec3 bloom = vec3(0.0);
    float totalWeight = 0.0;

    // Sample a 5x5 grid around the current pixel to approximate blur
    for (float x = -2.0; x <= 2.0; x += 1.0) {
        for (float y = -2.0; y <= 2.0; y += 1.0) {
            vec2 offset = vec2(x, y) * texelSize * bloomRadius;
            vec3 sampleColor = texture(texture0, uv + offset).rgb;

            float brightness = dot(sampleColor, vec3(0.299, 0.587, 0.114));
            float contribution = max(brightness - brightnessThreshold, 0.0);

            float weight = 1.0 / (1.0 + length(vec2(x, y)));

            bloom += sampleColor * contribution * weight;
            totalWeight += weight;
        }
    }

    bloom /= totalWeight;
    return bloom * bloomStrength;
}

void main()
{
    vec2 uv = fragTexCoord;
    vec4 texColor = texture(texture0, uv);
    vec3 color = texColor.rgb;

    // If background (alpha from RenderTexture is 0), use dynamic background colors from day cycle
    if (texColor.a < 0.1) {
        color = mix(bgBottom, bgTop, uv.y);
    } else {
        vec3 bloom = applyBloom(uv);
        color += bloom;
    }

    finalColor = vec4(color, 1.0);
}
