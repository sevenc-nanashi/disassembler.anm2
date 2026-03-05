cbuffer constant0 : register(b0) {
  float quantize_x;
  float quantize_y;
  float quantize_shift_x;
  float quantize_shift_y;
}

float4 quantize_grid(float4 pos : SV_Position, float2 uv : TEXCOORD)
    : SV_Target {
  // 量子化なしの場合はグリッドを表示しない
  if (quantize_x <= 1.0 && quantize_y <= 1.0) {
    return float4(0, 0, 0, 0);
  }
  uint check_x = (uint)((pos.x - quantize_shift_x) / quantize_x) % 2;
  uint check_y = (uint)((pos.y - quantize_shift_y) / quantize_y) % 2;
  float grid_color = quantize_x <= 1.0      ? (check_y == 0 ? 0.2 : 0.8)
                     : quantize_y <= 1.0    ? (check_x == 0 ? 0.2 : 0.8)
                     : (check_x == check_y) ? 0.2
                                            : 0.8;

  return float4(grid_color, grid_color, grid_color, 0.2);
}

// vim: set ft=hlsl ts=4 sts=4 sw=4 noet:
