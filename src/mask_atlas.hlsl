
cbuffer constant0 : register(b0) {
  float sourceWidth;
  float sourceHeight;
  float targetRed;
  float targetGreen;
  float targetBlue;
  float shiftX;
  float shiftY;
}

Texture2D srcTex : register(t0);
Texture2D atlasTex : register(t1);
SamplerState srcSmp : register(s0);

float4 mask_atlas(float4 pos : SV_Position, float2 uv : TEXCOORD) : SV_Target {
  int2 atlasPos = int2(pos.xy) + int2(shiftX, shiftY);
  float4 srcColor = srcTex.Load(int3(atlasPos, 0));
  if (srcColor.a < 0.01) {
    return float4(0, 0, 0, 0);
  }
  float4 atlasColor = atlasTex.Load(int3(atlasPos, 0));
  if (atlasColor.a < 0.01) {
    return float4(0, 0, 0, 0);
  }

  const float THRESHOLD = 1 / 64.0;
  float diffR = abs(atlasColor.r - targetRed);
  float diffG = abs(atlasColor.g - targetGreen);
  float diffB = abs(atlasColor.b - targetBlue);
  float mask =
      (diffR < THRESHOLD && diffG < THRESHOLD && diffB < THRESHOLD) ? 1.0 : 0.0;
  return srcColor * mask;
}

// vim: set ft=hlsl ts=4 sts=4 sw=4 noet:
