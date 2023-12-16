/*
 * 版权属于：yitter(yitter@126.com)
 * 开源地址：https://github.com/yitter/idgenerator
 */

pub struct IdGeneratorOptions {
    /// 雪花计算方法,（1-漂移算法|2-传统算法），默认1
    pub method: u8,

    /// 基础时间（ms单位），不能超过当前系统时间
    pub base_time: i64,

    /// 必须由外部设定，最大值 2^WorkerIdBitLength-1
    pub worker_id: u32,

    /// 默认值6，取值范围 [1, 15]（要求：序列数位长+机器码位长不超过22）
    pub worker_id_bit_length: u8,

    /// 默认值6，取值范围 [3, 21]（要求：序列数位长+机器码位长不超过22）
    pub seq_bit_length: u8,

    /// 设置范围 [MinSeqNumber, 2^SeqBitLength-1]，默认值0，表示最大序列数取最大值（2^SeqBitLength-1]）
    pub max_seq_number: u32,

    /// 默认值5，取值范围 [5, MaxSeqNumber]，每毫秒的前5个序列数对应编号0-4是保留位，其中1-4是时间回拨相应预留位，0是手工新值预留位
    pub min_seq_number: u32,

    /// 最大漂移次数（含），默认2000，推荐范围 500-20000（与计算能力有关）
    pub top_over_cost_count: u32,

    /// 生成流水序列之间的步长，默认为1。
    pub seq_step: u8,
}

impl IdGeneratorOptions {
    /// 创建新的生成器配置项，并赋予以下属性默认值：
    /// - method: 1。漂移算法。
    /// - base_time: 1582136402000。大约在`2020`年`2`月`19`日。
    /// - worker_id_bit_length: 6。机器码占据6位。
    /// - seq_bit_length: 6。序列数占据6位。
    /// - max_seq_number: 0。
    /// - min_seq_number: 5。
    /// - top_over_cost_count: 2000。
    /// - seq_step: 1。自增长序列步长。
    ///
    /// # Arguments
    ///
    /// * `worker_id`: 机器编号。
    ///
    /// returns: IdGeneratorOptions
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(worker_id: u32) -> IdGeneratorOptions {
        return IdGeneratorOptions {
            method: 1,
            worker_id,
            base_time: 1582136402000,
            worker_id_bit_length: 6,
            seq_bit_length: 6,
            max_seq_number: 0,
            min_seq_number: 5,
            top_over_cost_count: 2000,
            seq_step: 1,
        };
    }
}