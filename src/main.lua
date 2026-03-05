--label:配置
--information:https://github.com/parts_destruction.anm2

---$track:透明度閾値
---min=0
---max=100
---step=0.01
local threshold = 0

threshold = threshold / 100

---$check:中心位置を変更
local move_center = true

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

local internal = obj.module("parts_destruction")

local data, width, height = obj.getpixeldata("object")

local num_parts = internal.destruct(obj.effect_id, width, height, round(threshold * 255), data)
debug_dump("num_parts", num_parts)
obj.num = num_parts
local original_cx = obj.cx
local original_cy = obj.cy
local original_ox = obj.ox
local original_oy = obj.oy
debug_dump("original", { cx = original_cx, cy = original_cy, ox = original_ox, oy = original_oy })
for i = 0, num_parts - 1 do
    local dx, dy, pwidth, pheight, pdata = internal.get_part_image(obj.effect_id, i)
    debug_dump("part " .. i, { dx = dx, dy = dy, pwidth = pwidth, pheight = pheight })
    obj.index = i
    if move_center then
        obj.cx = 0
        obj.cy = 0
        obj.ox = dx + pwidth / 2 - original_cx - width / 2 + original_ox
        obj.oy = dy + pheight / 2 - original_cy - height / 2 + original_oy
    else
        obj.cx = -pwidth / 2 - dx + original_cx + width / 2
        obj.cy = -pheight / 2 - dy + original_cy + height / 2
    end
    obj.putpixeldata("object", pdata, pwidth, pheight)
    obj.effect()
    obj.draw()
    internal.dispose_part_image(pdata)
end

internal.dispose(obj.effect_id)