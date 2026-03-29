// 新增字段到 AppState 结构体
pub struct AppState {
    // ... 原版字段保留不变
    pub last_search: String,      // 上次查找内容
    pub last_replace: String,     // 上次替换内容
    pub show_help_bar: bool,      // Ctrl+L 切换：true=快捷键说明 / false=普通消息
    pub current_file_path: String,// 当前打开的文件路径
}

// 初始化默认值
impl Default for AppState {
    fn default() -> Self {
        Self {
            // ... 原版默认值保留不变
            last_search: String::new(),
            last_replace: String::new(),
            show_help_bar: true,
            current_file_path: String::new(),
        }
    }
}
