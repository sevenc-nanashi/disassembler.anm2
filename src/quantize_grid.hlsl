cbuffer constant0 : register(b0) {
  float quantize_x;
  float quantize_y;
  float quantize_shift_x;
  float quantize_shift_y;
}

uint checker(float pos, float quantize, float shift) {
  if (quantize <= 1.0) {
    return 0;
  }
  int cell = (int)floor((pos - shift) / quantize);
  return (uint)(cell & 1);
}

float4 quantize_grid(float4 pos : SV_Position, float2 uv : TEXCOORD)
    : SV_Target {
  // 量子化なしの場合はグリッドを表示しない
  if (quantize_x <= 1.0 && quantize_y <= 1.0) {
    return float4(0, 0, 0, 0);
  }
  uint check_x = checker(pos.x, quantize_x, quantize_shift_x);
  uint check_y = checker(pos.y, quantize_y, quantize_shift_y);
  // 片方が1の場合は帯にする
  float grid_color = quantize_x <= 1.0      ? (check_y == 0 ? 0.2 : 0.5)
                     : quantize_y <= 1.0    ? (check_x == 0 ? 0.2 : 0.5)
                     : (check_x == check_y) ? 0.2
                                            : 0.5;

  return float4(grid_color, grid_color, grid_color, 0.2);
}

// vim: set ft=hlsl ts=4 sts=4 sw=4 noet:
