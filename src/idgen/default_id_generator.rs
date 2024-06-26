/*
 * 版权属于：yitter(yitter@126.com)
 * 开源地址：https://github.com/yitter/idgenerator
 */
use crate::idgen::*;

// static mut instance2: Option<Arc<Mutex<SnowWorkerM1>>> = None;

pub struct DefaultIdGenerator {
    pub worker: SnowWorkerM1,
}

impl Default for DefaultIdGenerator {
    fn default() -> Self {
        DefaultIdGenerator { worker: SnowWorkerM1::default() }
    }
}

// impl DefaultIdGenerator {
//     pub fn default() -> DefaultIdGenerator {
//         DefaultIdGenerator { worker: SnowWorkerM1::default() }
//     }
// }
