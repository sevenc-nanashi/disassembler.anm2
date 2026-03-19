use std::ptr::NonNull;

use aviutl2::{anyhow, module::ScriptModuleFunctions, tracing};

mod sort;
mod union_find;

#[aviutl2::plugin(ScriptModule)]
struct DisassemblerMod2 {}

impl aviutl2::module::ScriptModule for DisassemblerMod2 {
    fn new(_info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        aviutl2::tracing_subscriber::fmt()
            .with_max_level(aviutl2::tracing::Level::DEBUG)
            .event_format(aviutl2::logger::AviUtl2Formatter)
            .with_writer(aviutl2::logger::AviUtl2LogWriter)
            .init();

        Ok(Self {})
    }
    fn plugin_info(&self) -> aviutl2::module::ScriptModuleTable {
        aviutl2::module::ScriptModuleTable {
            information: "disassembler.mod2 / Internal Module".to_string(),
            functions: Self::functions(),
        }
    }
}

struct SplatEntry {
    parts: Vec<SplatParts>,
}

pub(crate) struct SplatParts {
    pub(crate) dx: i64,
    pub(crate) dy: i64,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) key: (u8, u8, u8),
}

static SPLAT_DATA: std::sync::LazyLock<dashmap::DashMap<i32, SplatEntry>> =
    std::sync::LazyLock::new(dashmap::DashMap::new);

#[aviutl2::module::functions]
impl DisassemblerMod2 {
    #[allow(clippy::too_many_arguments)]
    fn destruct(
        &self,
        effect_id: i32,
        width: usize,
        height: usize,
        threshold: u8,
        sort_mode: i32,
        reference_point: i32,
        quantize_x: i32,
        quantize_y: i32,
        quantize_shift_x: i32,
        quantize_shift_y: i32,
        image_data: NonNull<u8>,
        return_image_data: NonNull<u8>,
    ) -> aviutl2::AnyResult<usize> {
        let start = std::time::Instant::now();
        let image_slice =
            unsafe { std::slice::from_raw_parts(image_data.as_ptr(), width * height * 4) };
        let return_image_slice = unsafe {
            std::slice::from_raw_parts_mut(return_image_data.as_ptr(), width * height * 4)
        };

        let pixel_count = width * height;
        let mut union_find = union_find::UnionFind::new(pixel_count);
        let mut active_indices = Vec::with_capacity(pixel_count / 4);
        for y in 0..height {
            let row_offset = y * width;
            for x in 0..width {
                let pixel_index = row_offset + x;
                let idx = pixel_index * 4;
                let alpha = image_slice[idx + 3];
                if alpha > threshold {
                    active_indices.push(pixel_index);
                    if x > 0 {
                        let left_alpha = image_slice[idx - 4 + 3];
                        if left_alpha > threshold {
                            union_find.union(pixel_index, pixel_index - 1);
                        }
                    }
                    if y > 0 {
                        let up_alpha = image_slice[idx - width * 4 + 3];
                        if up_alpha > threshold {
                            union_find.union(pixel_index, pixel_index - width);
                        }
                    }
                }
            }
        }

        let components = union_find.into_components_for_nodes(&active_indices);
        if components.is_empty() {
            tracing::debug!(
                "Effect ID {}: No active pixels found in the image.",
                effect_id
            );
            SPLAT_DATA.insert(effect_id, SplatEntry { parts: Vec::new() });
            return Ok(0);
        }
        const SPLIT_CHANNEL_INTO: usize = 64;
        const SHIFT_PER_CHANNEL: usize = 256 / SPLIT_CHANNEL_INTO;
        const MAX_PARTS: usize = SPLIT_CHANNEL_INTO * SPLIT_CHANNEL_INTO * SPLIT_CHANNEL_INTO;
        if components.len() > MAX_PARTS {
            return Err(anyhow::anyhow!(
                "Too many parts generated: {} (max {})",
                components.len(),
                MAX_PARTS
            ));
        }

        let mut splat_parts = Vec::with_capacity(components.len());
        for (i, indices) in components.into_iter().enumerate() {
            let mut min_x = width;
            let mut max_x = 0;
            let mut min_y = height;
            let mut max_y = 0;

            for idx in &indices {
                let x = idx % width;
                let y = idx / width;
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }

            let part_width = max_x - min_x + 1;
            let part_height = max_y - min_y + 1;

            let key = (
                (i % SPLIT_CHANNEL_INTO) as u8 * SHIFT_PER_CHANNEL as u8,
                ((i / SPLIT_CHANNEL_INTO) % SPLIT_CHANNEL_INTO) as u8 * SHIFT_PER_CHANNEL as u8,
                (i / (SPLIT_CHANNEL_INTO * SPLIT_CHANNEL_INTO)) as u8 * SHIFT_PER_CHANNEL as u8,
            );
            for idx in indices {
                return_image_slice[idx * 4] = key.0;
                return_image_slice[idx * 4 + 1] = key.1;
                return_image_slice[idx * 4 + 2] = key.2;
                return_image_slice[idx * 4 + 3] = 255;
            }

            splat_parts.push(SplatParts {
                dx: min_x as i64,
                dy: min_y as i64,
                width: part_width,
                height: part_height,
                key,
            });
        }
        let sort_config = sort::SortConfig {
            sort_mode,
            reference_point,
            quantize_x: i64::from(quantize_x.max(1)),
            quantize_y: i64::from(quantize_y.max(1)),
            quantize_shift_x: i64::from(quantize_shift_x),
            quantize_shift_y: i64::from(quantize_shift_y),
            image_width: width as i64,
            image_height: height as i64,
        };
        sort::sort_parts(&mut splat_parts, sort_config);

        let length = splat_parts.len();
        SPLAT_DATA.insert(effect_id, SplatEntry { parts: splat_parts });

        tracing::debug!(
            "Effect ID {}: Destructed {}x{} into {} parts in {:.2?}",
            effect_id,
            width,
            height,
            length,
            start.elapsed()
        );
        Ok(length)
    }

    fn get_part_image_info(
        &self,
        effect_id: i32,
    ) -> aviutl2::AnyResult<(i64, i64, usize, usize, u8, u8, u8)> {
        let mut entry = SPLAT_DATA
            .get_mut(&effect_id)
            .ok_or_else(|| anyhow::anyhow!("Effect ID not found"))?;
        let part = entry
            .parts
            .pop()
            .ok_or_else(|| anyhow::anyhow!("No parts available for this effect ID"))?;

        Ok((
            part.dx,
            part.dy,
            part.width,
            part.height,
            part.key.0,
            part.key.1,
            part.key.2,
        ))
    }

    fn dispose(&self, effect_id: i32) -> aviutl2::AnyResult<()> {
        SPLAT_DATA.remove(&effect_id);
        Ok(())
    }
}

aviutl2::register_script_module!(DisassemblerMod2);
