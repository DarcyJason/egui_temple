# 常用的UI组件

## 1. 标签

```
ui.label("This is a label");
```

## 2. 按钮

```
if ui.button("click me!").clicked() {
    println!("Button clicked!")
}
```

## 3. 文本框

### 3.1 单行文本框

```
ui.text_edit_singleline(&mut self.something);
```

### 3.2 多行文本框

```
ui.text_edit_multiline(&mut self.something);
```

## 4. 滑动条

```
ui.add(egui::Sider::new(&mut self.something, 0..=1000).text("Something"));
```

## 5. 复选框
```
let mut is_checked = true;
ui.checkbox(&mut is_checked, "Enable feature");
```

## 6. 单选按钮

```
let mut selected = 1;
ui.radio_value(&mut selected, 1, "Option 1");
ui.radio_value(&mur selected, 2, "Option 2");
```

## 7. 下拉框

```
let mut selected = "Option 1";
egui::ComboBox::from_label("Select an option: ")
    .selected_text(selected)
    .show_ui(ui, |ui| {
        ui.selectable_value(&mut selected, "Option 1", "Option 1");
        ui.selectable_value(&mut selected, "Option 2", "Option 2");
});
```

## 8. 进度条

```
let progress = 0.5;
ui.add(egui::ProgressBar::new(progress).text("Loading..."));
```

## 9. 分割线

```
ui.separator();
```

## 10. 可折叠标题

```
egui::CollapsingHeader::new("More details")
    .show(ui, |ui| {
        ui.label("Here are the details...");
});
```

## 11. 图像

```
let texture_id = ...
ui.image(texture_id, [100.0, 100.0]);
```

## 12. 超链接

```
ui.hyperlink("http://www.egui.rs");
```

## 13. 拖动数值

```
let mut value = 42;
ui.add(egui::DragValue::new(&mut value));
```

## 14. 工具提示

```
ui.label("Hover me for tooltip")
    .on_hover_text("This is a tooltip");
```

## 15. 水平布局

```
ui.horizontal(|ui| {
    ui.label("Name:");
    ui.text_edit_singleline(&mut name);
});
```

## 16. 垂直布局

```
ui.vertical(|ui| {
    ui.label("Name:");
    ui.text_edit_singleline(&mut name);
});
```

## 17. 网络布局

```
ui.grid(|ui| {
    ui.label("Row 1, Column 1");
    ui.label("Row 1, Column 2");
    ui.end_row();
    ui.label("Row 2, Column 1");
    ui.label("Row 2, Column 2");
});
```

## 18. 水平垂直都居中

```
let available_height = ui.available_height();
let padding = available_height * 0.25;
ui.vertical_centered(|ui| {
    ui.add_space(padding);
    ui.label("The content is on center!");
    ui.add_space(padding);
});
```

```
