#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform float uTime;

out vec4 finalColor;

void main()
{
    vec4 texColor = texture(texture0, fragTexCoord) * fragColor;
    float pulse = sin(uTime * 3.0) * 0.5 + 0.5;

    vec4 redColor = vec4(1.0, 0.0, 0.0, 1.0);
    finalColor = mix(texColor, redColor, pulse);
}
