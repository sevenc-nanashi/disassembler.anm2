use std::ptr::NonNull;

use aviutl2::{anyhow, module::ScriptModuleFunctions, tracing};

mod union_find;

#[aviutl2::plugin(ScriptModule)]
struct PartsDestructionMod2 {}

impl aviutl2::module::ScriptModule for PartsDestructionMod2 {
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
            information: "parts_destruction.mod2 / Internal Module".to_string(),
            functions: Self::functions(),
        }
    }
}

struct SplatEntry {
    parts: Vec<SplatParts>,
}

struct SplatParts {
    dx: i64,
    dy: i64,
    width: usize,
    height: usize,
    image: Vec<u8>,
}

static SPLAT_DATA: std::sync::LazyLock<dashmap::DashMap<i32, SplatEntry>> =
    std::sync::LazyLock::new(dashmap::DashMap::new);
static SPLAT_IMAGE_POINTERS: std::sync::LazyLock<dashmap::DashMap<usize, Vec<u8>>> =
    std::sync::LazyLock::new(dashmap::DashMap::new);

#[aviutl2::module::functions]
impl PartsDestructionMod2 {
    fn destruct(
        &self,
        effect_id: i32,
        width: usize,
        height: usize,
        threshold: u8,
        image_data: NonNull<u8>,
    ) -> aviutl2::AnyResult<usize> {
        let start = std::time::Instant::now();
        let image_slice =
            unsafe { std::slice::from_raw_parts(image_data.as_ptr(), width * height * 4) };

        let mut union_find = union_find::UnionFind::new(width * height);
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 4;
                let alpha = image_slice[idx + 3];
                if alpha > threshold {
                    if x > 0 {
                        let left_alpha = image_slice[idx - 4 + 3];
                        if left_alpha > threshold {
                            union_find.union(y * width + x, y * width + (x - 1));
                        }
                    }
                    if y > 0 {
                        let up_alpha = image_slice[idx - width * 4 + 3];
                        if up_alpha > threshold {
                            union_find.union(y * width + x, (y - 1) * width + x);
                        }
                    }
                }
            }
        }

        let components = union_find.into_components();

        let mut splat_parts = Vec::new();
        for indices in components {
            if indices.len() == 1 && image_slice[indices[0] * 4 + 3] <= threshold {
                continue; // Skip single transparent pixels
            }
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
            let mut part_image = vec![0u8; part_width * part_height * 4];

            for idx in indices {
                let x = idx % width;
                let y = idx / width;
                let part_idx = ((y - min_y) * part_width + (x - min_x)) * 4;
                let original_idx = idx * 4;
                part_image[part_idx..part_idx + 4]
                    .copy_from_slice(&image_slice[original_idx..original_idx + 4]);
            }

            splat_parts.push(SplatParts {
                dx: min_x as i64,
                dy: min_y as i64,
                width: part_width,
                height: part_height,
                image: part_image,
            });
        }
        splat_parts.sort_by_key(|part| (part.dx, part.dy));

        let length = splat_parts.len();
        SPLAT_DATA.insert(effect_id, SplatEntry { parts: splat_parts });

        tracing::debug!(
            "Effect ID {}: Destructed {}x{} into {} parts in {:.2?}",
            width,
            height,
            effect_id,
            length,
            start.elapsed()
        );
        Ok(length)
    }

    fn get_part_image(
        &self,
        effect_id: i32,
        part_index: usize,
    ) -> aviutl2::AnyResult<(i64, i64, usize, usize, *const u8)> {
        let entry = SPLAT_DATA
            .get(&effect_id)
            .ok_or_else(|| anyhow::anyhow!("Effect ID not found"))?;
        let part = entry
            .parts
            .get(part_index)
            .ok_or_else(|| anyhow::anyhow!("Part index out of range"))?;

        let cloned_image = part.image.clone();
        let ptr = cloned_image.as_ptr();
        SPLAT_IMAGE_POINTERS.insert(ptr as usize, cloned_image);

        Ok((part.dx, part.dy, part.width, part.height, ptr))
    }

    fn dispose_part_image(&self, image_ptr: NonNull<u8>) -> aviutl2::AnyResult<()> {
        SPLAT_IMAGE_POINTERS
            .remove(&(image_ptr.as_ptr() as usize))
            .ok_or_else(|| anyhow::anyhow!("Image pointer not found"))?;
        Ok(())
    }

    fn dispose(&self, effect_id: i32) -> aviutl2::AnyResult<()> {
        SPLAT_DATA.remove(&effect_id);
        Ok(())
    }
}

aviutl2::register_script_module!(PartsDestructionMod2);
