# 项目介绍

本项目实现了一个支持显示字符串应用的简单操作系统，其中根据实验要求已经实现的功能包括：

1. 实现基本的`println`，以实现在裸机上输出`hello world`字符串
2. 实现输出的**分级优先级控制**，对于ERROR、WARN、INFO、DEBUG和TRACE五个等级的输出进行管理，并可以通过`make run LOG=***`来进行自主选择
3. 对于优先级不同的输出文本实现了彩色输出
4. 在INFO等级下输出了os的内存分布布局，对于`.text`、`.data`、`.rodata`、`.bss`段位置信息进行输出

# 运行方法

在os目录下：

`make run`以默认INFO等级运行

`make run LOG=ERROR`或者`LOG=ERROR make run`在ERROR或者其他等级下运行

