# RGB 和 HSL 颜色空间转换原理

## 一、RGB to HSL 转换

RGB 表示法使用红、绿、蓝三个分量直接描述颜色，而 HSL 表示法则使用色相(Hue)、饱和度(Saturation)和亮度(Lightness)来描述颜色。转换过程需要以下步骤：

### 1. 归一化
将 RGB 值归一化到 [0.0, 1.0] 范围内：
```
R' = R / 255
G' = G / 255
B' = B / 255
```

### 2. 计算最值
找出归一化后 RGB 值的最大值和最小值：
```
max = max(R', G', B')
min = min(R', G', B')
delta = max - min
```

### 3. 计算亮度 (L)
```
L = (max + min) / 2
```

### 4. 计算饱和度 (S)
```
if max == min {
    S = 0  // 灰度颜色
} else {
    if L <= 0.5 {
        S = delta / (max + min)
    } else {
        S = delta / (2.0 - max - min)
    }
}
```

### 5. 计算色相 (H)
```
if max == min {
    H = 0  // 灰度颜色
} else {
    H = match max {
        R' => (G' - B') / delta + (if G' < B' { 6.0 } else { 0.0 })
        G' => (B' - R') / delta + 2.0
        B' => (R' - G') / delta + 4.0
    }
    H *= 60  // 转换到度数 [0, 360)
}
```

## 二、HSL to RGB 转换

将 HSL 颜色空间转换回 RGB 颜色空间的过程相对更复杂。

### 1. 计算中间值
```
h = H / 360  // 将色相归一化到 [0, 1]
```

### 2. 计算色度 (Chroma)
```
C = (1 - |2L - 1|) * S
```

### 3. 计算中间色调
```
X = C * (1 - |(h * 6) mod 2 - 1|)
m = L - C/2
```

### 4. 根据 h 计算 RGB' 值
```
(R', G', B') = match h {
    h < 1/6   => (C, X, 0)
    h < 2/6   => (X, C, 0)
    h < 3/6   => (0, C, X)
    h < 4/6   => (0, X, C)
    h < 5/6   => (X, 0, C)
    else      => (C, 0, X)
}
```

### 5. 计算最终 RGB 值
```
R = (R' + m) * 255
G = (G' + m) * 255
B = (B' + m) * 255
```

## 三、Rust 实现示例

```rust
pub struct Rgb {
    r: f64,
    g: f64,
    b: f64,
}

pub struct Hsl {
    h: f64,  // 0-360
    s: f64,  // 0-1
    l: f64,  // 0-1
}

impl Rgb {
    pub fn to_hsl(&self) -> Hsl {
        // 实现 RGB 到 HSL 的转换
        // 按照上述步骤 1-5 实现
    }
}

impl Hsl {
    pub fn to_rgb(&self) -> Rgb {
        // 实现 HSL 到 RGB 的转换
        // 按照上述步骤 1-5 实现
    }
}
```

## 四、注意事项

1. 浮点数计算可能存在精度误差，在比较颜色值时应该使用适当的容差。

2. RGB 值通常使用 0-255 范围，而 HSL 中：
   - H: 0-360 度
   - S: 0-1 (或 0%-100%)
   - L: 0-1 (或 0%-100%)

3. 转换过程中的中间计算都应该使用浮点数以保持精度。

4. 在实际应用中，建议使用经过验证的颜色转换库，因为它们通常经过优化并处理了边界情况。

## 五、参考资料

- [W3C Color Module Level 4](https://www.w3.org/TR/css-color-4/)
- [RGB to HSL and HSL to RGB Color Conversion Algorithms](https://www.niwa.nu/2013/05/math-behind-colorspace-conversions-rgb-hsl/)

这些转换原理在处理颜色时非常重要，尤其是在需要调整颜色的亮度、饱和度或者生成颜色主题时。理解这些原理有助于更好地控制和操作颜色。
