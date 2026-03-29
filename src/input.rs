// handle_keypress 函数中新增匹配项
match key {
    // ... 原版快捷键保留不变
    
    // Ctrl+H 查找替换
    Key::Ctrl('h') => {
        self.state.mode = Mode::SearchReplace;
        self.status_bar.set_message("查找替换: 输入查找内容".to_string());
    }
    
    // Ctrl+L 切换底部栏
    Key::Ctrl('l') => {
        self.state.show_help_bar = !self.state.show_help_bar;
        self.status_bar.toggle_help();
    }
    
    // Ctrl+O 打开文件
    Key::Ctrl('o') => {
        self.state.mode = Mode::OpenFile;
        self.status_bar.set_message("打开/新建文件: 输入路径".to_string());
    }
    
    // Ctrl+Q 退出（带保存提示）
    Key::Ctrl('q') => {
        self.handle_quit_with_save_prompt();
    }
}
