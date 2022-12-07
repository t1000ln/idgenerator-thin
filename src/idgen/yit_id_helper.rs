/*
 * 版权属于：yitter(yitter@126.com)
 * 开源地址：https://github.com/yitter/idgenerator
 */
use std::sync::Mutex;
use std::sync::Arc;
use crate::idgen::*;

pub struct YitIdHelper;

static mut ID_GEN_INSTANCE: Option<Arc<Mutex<DefaultIdGenerator>>> = None;

impl YitIdHelper {
    fn id_gen_instance() -> Arc<Mutex<DefaultIdGenerator>> {
        unsafe {
            ID_GEN_INSTANCE.get_or_insert_with(|| {
                Arc::new(Mutex::new(DefaultIdGenerator::default()))
            }).clone()
        }
    }

    pub fn set_id_generator(options: IdGeneratorOptions) {
        let idgen_arc = YitIdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.set_options(options);
    }

    pub fn set_worker_id(worker_id: u32) {
        let idgen_arc = YitIdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        let options = IdGeneratorOptions::new(worker_id);
        idgen.worker.set_options(options);
    }

    pub fn next_id() -> i64 {
        let idgen_arc = YitIdHelper::id_gen_instance();
        let mut idgen = idgen_arc.lock().unwrap();
        idgen.worker.next_id()
    }
}
