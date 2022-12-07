﻿# idgenerator-thin
修改自 https://github.com/yitter/IdGenerator 的项目。

对源代码中所有的变量名和函数名进行重命名以符合`Rust`风格，去掉无用的代码，去掉`pub extern "C" fn`部分。

## 调用示例（Rust）

第1步，**全局** 初始化（应用程序启动时执行一次）：
```
use idgenerator-thin::{IdGeneratorOptions, YitIdHelper};

// 创建 IdGeneratorOptions 对象，请在构造函数中输入 worker_id：
let options = IdGeneratorOptions::new(1);
// options.worker_id_bit_length = 10; // worker_id_bit_length 默认值6，支持的 worker_id 最大值为2^6-1，若 worker_id 超过64，可设置更大的 worker_id_bit_length
// ...... 其它参数设置参考 IdGeneratorOptions 定义，一般来说，只要再设置 worker_id_bit_length （决定 worker_id 的最大值）。

// 保存参数（必须的操作，否则以上设置都不能生效）：
YitIdHelper::set_id_generator(options);
// 以上初始化过程只需全局一次，且必须在第2步之前设置。
```

第2步，生成ID：
```
// 初始化以后，即可在任何需要生成ID的地方，调用以下方法：
let next_id = YitIdHelper::next_id();
println!("next_id: {}", next_id);
```

