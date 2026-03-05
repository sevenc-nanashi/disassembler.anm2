--label:配置
--information:https://github.com/disassembler.anm2

--[[pixelshader@quantize_grid:
---$include "./quantize_grid.hlsl"
]]

---$track:透明度閾値
---min=0
---max=100
---step=0.1
local threshold = 25

threshold = threshold / 100

---$check:中心位置を変更
local move_center = true

--group:ソート,false

---$select:ソート方向
---X-→X+ / Y-→Y+=0
---X-→X+ / Y+→Y-=1
---X+→X- / Y-→Y+=2
---X+→X- / Y+→Y-=3
---Y-→Y+ / X-→X+=4
---Y-→Y+ / X+→X-=5
---Y+→Y- / X-→X+=6
---Y+→Y- / X+→X-=7
---Z（左上→右下 / x + w*y）=8
---Z（右下→左上 / -(x + w*y)）=9
---逆Z（右上→左下 / -x + w*y）=10
---逆Z（左下→右上 / x - w*y）=11
---N（右上→左下 / -x*h + y）=12
---N（左下→右上 / x*h - y）=13
---逆N（右下→左上 / -(x*h + y)）=14
---逆N（左上→右下 / x*h + y）=15
local sort_mode = 0

---$select:基準座標
---左上=0
---上=1
---右上=2
---左=3
---中心=4
---右=5
---左下=6
---下=7
---右下=8
local reference_point = 4

---$track:X量子化
---min=1
---max=256
---step=1
local quantize_x = 1

---$track:Y量子化
---min=1
---max=256
---step=1
local quantize_y = 1

---$track:X量子化シフト
---min=-256
---max=256
---step=1
local quantize_shift_x = 0

---$track:Y量子化シフト
---min=-256
---max=256
---step=1
local quantize_shift_y = 0

---$check:分解パーツの可視化
local visualize_parts = false

-- 出力中は可視化を無効にする
visualize_parts = visualize_parts and obj.getoption("saving")

---$select:可視化時の色
---グラデーション=0
---ランダム=1
local visualize_color_mode = 0


--group:高度な設定,false

---$check:デバッグモード
local debug = false

---$value:PI
local PI = {}

-- PIからパラメータを取得
if type(PI.threshold) == "number" then
    threshold = PI.threshold
end
if type(PI.debug) == "boolean" then
    debug = PI.debug
end
if type(PI.sort_mode) == "number" then
    sort_mode = PI.sort_mode
end
if type(PI.reference_point) == "number" then
    reference_point = PI.reference_point
end
if type(PI.quantize_x) == "number" then
    quantize_x = PI.quantize_x
end
if type(PI.quantize_y) == "number" then
    quantize_y = PI.quantize_y
end
if type(PI.quantize_shift_x) == "number" then
    quantize_shift_x = PI.quantize_shift_x
end
if type(PI.quantize_shift_y) == "number" then
    quantize_shift_y = PI.quantize_shift_y
end
if type(PI.show_quantize_grid) == "boolean" then
    visualize_parts = PI.show_quantize_grid
end

-- デバッグ用関数
local function debug_dump_internal(o)
    if type(o) == "table" then
        local s = "{ "
        local keys = {}
        local is_array = true
        local max_index = 0
        for k, _ in pairs(o) do
            table.insert(keys, k)
            if type(k) ~= "number" or k < 1 or math.floor(k) ~= k then
                is_array = false
            else
                if k > max_index then
                    max_index = k
                end
            end
        end
        if is_array then
            table.sort(keys, function(a, b)
                return a < b
            end)
        else
            table.sort(keys, function(a, b)
                return tostring(a) < tostring(b)
            end)
        end
        for i, k in ipairs(keys) do
            local v = o[k]
            if i > 1 then
                s = s .. ", "
            end
            if is_array then
                s = s .. debug_dump_internal(v)
            else
                s = s .. tostring(k) .. " = " .. debug_dump_internal(v)
            end
        end

        return s .. " }"
    elseif type(o) == "string" then
        return string.format("%q", o)
    else
        return tostring(o)
    end
end
local function debug_dump(m, o)
    if debug then
        if o == nil then
            debug_print(debug_dump_internal(m))
        else
            debug_print(m .. ": " .. debug_dump_internal(o))
        end
    end
end

local function round(num)
    return math.floor(num + 0.5)
end

local internal = obj.module("disassembler")

local data, width, height = obj.getpixeldata("object")
local quantize_x_int = math.max(1, round(quantize_x))
local quantize_y_int = math.max(1, round(quantize_y))
local quantize_shift_x_int = round(quantize_shift_x)
local quantize_shift_y_int = round(quantize_shift_y)

local num_parts = internal.destruct(
    obj.effect_id,
    width,
    height,
    round(threshold * 255),
    sort_mode,
    reference_point,
    quantize_x_int,
    quantize_y_int,
    quantize_shift_x_int,
    quantize_shift_y_int,
    data
)
if num_parts == 0 then
    return
end
debug_dump("num_parts", num_parts)
obj.num = num_parts
local oobj = {}
for k, v in pairs(obj) do
    oobj[k] = v
end

local function reset_object()
    obj.cx = oobj.cx
    obj.cy = oobj.cy
    obj.cz = oobj.cz
    obj.ox = oobj.ox
    obj.oy = oobj.oy
    obj.oz = oobj.oz
    obj.sx = oobj.sx
    obj.sy = oobj.sy
    obj.sz = oobj.sz
    obj.rx = oobj.rx
    obj.ry = oobj.ry
    obj.rz = oobj.rz
    obj.zoom = oobj.zoom
    obj.aspect = oobj.aspect
    obj.alpha = oobj.alpha
end

debug_dump("original",
    {
        cx = oobj.cx,
        cy = obj.cy,
        cz = obj.cz,
        ox = obj.ox,
        oy = obj.oy,
        oz = obj.oz,
    }
)

local hues = {}
for i = 0, num_parts - 1 do
    hues[i] = i * 360 / num_parts
end

if visualize_color_mode == 1 then
    -- シャッフル
    for i = num_parts - 1, 1, -1 do
        local j = obj.rand(0, i, i, obj.getvalue("frame_s"))
        hues[i], hues[j] = hues[j], hues[i]
    end
end

local cache_name = ("cache:disassembler_%d"):format(obj.effect_id)
if visualize_parts then
    obj.setoption("drawtarget", "tempbuffer", width, height)
    obj.pixelshader("quantize_grid", "tempbuffer", {}, {
        quantize_x,
        quantize_y,
        quantize_shift_x,
        quantize_shift_y,
    })
    _ = obj.copybuffer(cache_name, "tempbuffer")
    obj.setoption("drawtarget", "framebuffer")
end
for i = 0, num_parts - 1 do
    local dx, dy, pwidth, pheight, pdata = internal.get_part_image(obj.effect_id)
    debug_dump("part " .. i, { dx = dx, dy = dy, pwidth = pwidth, pheight = pheight })
    reset_object()
    obj.index = i
    obj.putpixeldata("object", pdata, pwidth, pheight)
    if move_center then
        obj.cx = 0
        obj.cy = 0
        obj.ox = dx + pwidth / 2 - oobj.cx - width / 2 + oobj.ox
        obj.oy = dy + pheight / 2 - oobj.cy - height / 2 + oobj.oy
    else
        obj.cx = -pwidth / 2 - dx + oobj.cx + width / 2
        obj.cy = -pheight / 2 - dy + oobj.cy + height / 2
        obj.ox = oobj.ox
        obj.oy = oobj.oy
    end
    obj.effect()
    obj.draw()

    if visualize_parts then
        obj.setoption("drawtarget", "tempbuffer")
        _ = obj.copybuffer("tempbuffer", cache_name)
        local hue = hues[i]
        local debug_color = HSV(hue, 50, 100)
        obj.effect("単色化", "強さ", 100, "色", debug_color, "輝度を保持する", "0")
        obj.draw(-width / 2 + dx + pwidth / 2, -height / 2 + dy + pheight / 2)
        local circle_x, circle_y
        if reference_point == 0 then
            circle_x, circle_y = dx, dy
        elseif reference_point == 1 then
            circle_x, circle_y = dx + pwidth / 2, dy
        elseif reference_point == 2 then
            circle_x, circle_y = dx + pwidth, dy
        elseif reference_point == 3 then
            circle_x, circle_y = dx, dy + pheight / 2
        elseif reference_point == 4 then
            circle_x, circle_y = dx + pwidth / 2, dy + pheight / 2
        elseif reference_point == 5 then
            circle_x, circle_y = dx + pwidth, dy + pheight / 2
        elseif reference_point == 6 then
            circle_x, circle_y = dx, dy + pheight
        elseif reference_point == 7 then
            circle_x, circle_y = dx + pwidth / 2, dy + pheight
        elseif reference_point == 8 then
            circle_x, circle_y = dx + pwidth, dy + pheight
        end
        local circle_color = HSV(hue, 100, 100)

        _ = obj.load("figure", "円", circle_color, 5, 100)
        obj.draw(
            circle_x - width / 2,
            circle_y - height / 2
        )
        _ = obj.copybuffer(cache_name, "tempbuffer")
        obj.setoption("drawtarget", "framebuffer")
    end
    internal.dispose_part_image(pdata)
end

internal.dispose(obj.effect_id)

if visualize_parts then
    _ = obj.copybuffer("tempbuffer", cache_name)
    obj.load("tempbuffer")
    reset_object()
    debug_dump("visualize_parts", oobj)
    obj.draw()
end