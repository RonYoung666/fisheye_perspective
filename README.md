# fisheye_perspective
把平面图像转成鱼眼透视图像

**使用方法：** cargo run \<File> \<width> \<distance>
* File：图像文件名
* width：图像宽度
* distance：眼睛距离图像的距离

**举例：** cargo run grid.png 1 0.5

表示 grid.png 宽 1 米，在距离 0.5 米的位置来观察它。

## gird.png：

![grid](https://user-images.githubusercontent.com/25758754/230038513-fe38435c-ba6e-4c07-aee4-b11eefa10d13.png)

## 输出结果（黑色表示在 180° 视野之外）：

![out_1_0 5_2023-04-05_014622](https://user-images.githubusercontent.com/25758754/230038616-571b10c0-01ae-4a22-8bc1-2d90d205f38b.png)
