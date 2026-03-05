Texture2D inputTexture : register(t0);
SamplerState samplerState : register(s0);

cbuffer constant0 : register(b0) {
    float center_x;
    float center_y;
	float red;
	float green;
	float blue;
}

float4 draw_circle(float4 pos : SV_Position, float2 uv : TEXCOORD) : SV_Target {
    float2 pixel_pos = pos.xy;
	float2 dir = pixel_pos - float2(center_x, center_y);
	float dist = length(dir);
	float radius = 4.0; // Circle radius

	if (dist < radius) {
		return float4(red, green, blue, 1.0);
	} else {
		return inputTexture.Sample(samplerState, uv);
	}
}

// vim: set ft=hlsl ts=4 sts=4 sw=4 noet:
