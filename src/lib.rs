pub mod idgen;
pub use idgen::*;

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use crate::{IdGeneratorOptions, YitIdHelper};

    #[test]
    fn test_next_id() {
        // 创建 IdGeneratorOptions 对象，请在构造函数中输入 WorkerId：
        let mut options = IdGeneratorOptions::new(1);
        options.seq_step = 10;
        // options.WorkerIdBitLength = 10; // WorkerIdBitLength 默认值6，支持的 WorkerId 最大值为2^6-1，若 WorkerId 超过64，可设置更大的 WorkerIdBitLength
        // ...... 其它参数设置参考 IdGeneratorOptions 定义，一般来说，只要再设置 WorkerIdBitLength （决定 WorkerId 的最大值）。

        let base_datetime = DateTime::<Utc>::from_timestamp(options.base_time / 1000, 0).unwrap();
        println!("{:?}", base_datetime);

        // 保存参数（必须的操作，否则以上设置都不能生效）：
        YitIdHelper::set_id_generator(options);
        // 以上初始化过程只需全局一次，且必须在第2步之前设置。

        // 初始化以后，即可在任何需要生成ID的地方，调用以下方法：
        for _ in 0..100 {
            let next_id = YitIdHelper::next_id();
            println!("next_id: {}", next_id);
            assert!(next_id > 0);
        }
    }
}