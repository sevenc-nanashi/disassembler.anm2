use crate::SplatParts;

#[derive(Clone, Copy)]
pub(crate) struct SortConfig {
    pub(crate) sort_mode: i32,
    pub(crate) reference_point: i32,
    pub(crate) quantize_x: i64,
    pub(crate) quantize_y: i64,
    pub(crate) quantize_shift_x: i64,
    pub(crate) quantize_shift_y: i64,
    pub(crate) image_width: i64,
    pub(crate) image_height: i64,
}

fn reference_position_x2(part: &SplatParts, reference_point: i32) -> i64 {
    let x2 = part.dx * 2;
    let w = part.width as i64;
    match reference_point {
        1 => x2 + w,
        2 => x2 + w * 2,
        3 => x2,
        4 => x2 + w,
        5 => x2 + w * 2,
        6 => x2,
        7 => x2 + w,
        8 => x2 + w * 2,
        _ => x2,
    }
}

fn reference_position_y2(part: &SplatParts, reference_point: i32) -> i64 {
    let y2 = part.dy * 2;
    let h = part.height as i64;
    match reference_point {
        1 => y2,
        2 => y2,
        3 => y2 + h,
        4 => y2 + h,
        5 => y2 + h,
        6 => y2 + h * 2,
        7 => y2 + h * 2,
        8 => y2 + h * 2,
        _ => y2,
    }
}

fn cmp_with_order(a: i64, b: i64, asc: bool) -> std::cmp::Ordering {
    if asc { a.cmp(&b) } else { b.cmp(&a) }
}

fn quantize_floor_with_shift(value: i64, step: i64, shift: i64) -> i64 {
    if step <= 1 {
        return value;
    }
    ((value - shift).div_euclid(step)) * step + shift
}

struct SortKeys {
    rx: i64,
    ry: i64,
    z: i64,
    rev_z: i64,
    n: i64,
    rev_n: i64,
    n_flip_y: i64,
    rev_n_flip_y: i64,
}

fn part_scan_keys(part: &SplatParts, config: SortConfig) -> SortKeys {
    let rx = quantize_floor_with_shift(
        reference_position_x2(part, config.reference_point),
        config.quantize_x * 2,
        config.quantize_shift_x * 2,
    );
    let ry = quantize_floor_with_shift(
        reference_position_y2(part, config.reference_point),
        config.quantize_y * 2,
        config.quantize_shift_y * 2,
    );
    SortKeys {
        rx,
        ry,
        z: rx + config.image_width * ry,
        rev_z: -rx + config.image_width * ry,
        n: rx * config.image_height + ry,
        rev_n: -rx * config.image_height + ry,
        n_flip_y: rx * config.image_height - ry,
        rev_n_flip_y: -rx * config.image_height - ry,
    }
}

fn compare_parts(a: &SplatParts, b: &SplatParts, config: SortConfig) -> std::cmp::Ordering {
    let ak = part_scan_keys(a, config);
    let bk = part_scan_keys(b, config);
    match config.sort_mode {
        0 => cmp_with_order(ak.rx, bk.rx, true).then_with(|| cmp_with_order(ak.ry, bk.ry, true)),
        1 => cmp_with_order(ak.rx, bk.rx, true).then_with(|| cmp_with_order(ak.ry, bk.ry, false)),
        2 => cmp_with_order(ak.rx, bk.rx, false).then_with(|| cmp_with_order(ak.ry, bk.ry, true)),
        3 => cmp_with_order(ak.rx, bk.rx, false).then_with(|| cmp_with_order(ak.ry, bk.ry, false)),
        4 => cmp_with_order(ak.ry, bk.ry, true).then_with(|| cmp_with_order(ak.rx, bk.rx, true)),
        5 => cmp_with_order(ak.ry, bk.ry, true).then_with(|| cmp_with_order(ak.rx, bk.rx, false)),
        6 => cmp_with_order(ak.ry, bk.ry, false).then_with(|| cmp_with_order(ak.rx, bk.rx, true)),
        7 => cmp_with_order(ak.ry, bk.ry, false).then_with(|| cmp_with_order(ak.rx, bk.rx, false)),
        8 => cmp_with_order(ak.z, bk.z, true),
        9 => cmp_with_order(ak.z, bk.z, false),
        10 => cmp_with_order(ak.rev_z, bk.rev_z, true),
        11 => cmp_with_order(ak.rev_z, bk.rev_z, false),
        12 => cmp_with_order(ak.rev_n, bk.rev_n, true),
        13 => cmp_with_order(ak.n_flip_y, bk.n_flip_y, true),
        14 => cmp_with_order(ak.rev_n_flip_y, bk.rev_n_flip_y, true),
        15 => cmp_with_order(ak.n, bk.n, true),
        _ => cmp_with_order(ak.rx, bk.rx, true).then_with(|| cmp_with_order(ak.ry, bk.ry, true)),
    }
    .then_with(|| cmp_with_order(a.dx, b.dx, true))
    .then_with(|| cmp_with_order(a.dy, b.dy, true))
    .then_with(|| cmp_with_order(a.width as i64, b.width as i64, true))
    .then_with(|| cmp_with_order(a.height as i64, b.height as i64, true))
}

pub(crate) fn sort_parts(parts: &mut [SplatParts], config: SortConfig) {
    parts.sort_by(|a, b| compare_parts(a, b, config));
    parts.reverse();
}
