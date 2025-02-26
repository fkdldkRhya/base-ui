#version 330 core
in vec2 TexCoord;
out vec4 FragColor;
uniform sampler2D ourTexture;
uniform vec4 textColor;

void main() {
    float alpha = texture(ourTexture, TexCoord).r;
    FragColor = vec4(textColor.rgb, textColor.a * alpha);
} 