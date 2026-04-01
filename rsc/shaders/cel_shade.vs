#version 330

in vec3 vertexPosition;
in vec3 vertexNormal;
in vec2 vertexTexCoord;
in vec4 vertexColor;

uniform mat4 matModel;
uniform mat4 matView;
uniform mat4 matProjection;
uniform mat4 matNormal;

out vec3 fragPos;
out vec3 fragNormal;
out vec2 fragTexCoord;
out vec4 fragColor;

void main()
{
    // World-space position
    fragPos = vec3(matModel * vec4(vertexPosition, 1.0));

    // World-space normal
    fragNormal = normalize(mat3(transpose(inverse(matModel))) * vertexNormal);

    fragTexCoord = vertexTexCoord;
    fragColor = vertexColor;

    gl_Position = matProjection * matView * vec4(fragPos, 1.0);
}
