
# Rust 中使用 Tonic 构建 grpc 服务

Tonic 构建grpc 服务的优势

- automated protobuf complilation
- macros for protobuf importing
- compile time contract checks


### 编写proto协议文件

```proto

syntax = "proto3";

package calculator;

service Calculator {
  rpc Add(CalculationRequest) returns(CalculationResponse);
}

message CalculationRequest {
  int64 a = 1;
  int64 b = 2;
}

message CalculationResponse {
  int64 result = 1;
}
```

### 安装protoc 编译器

```shell
sudo pacman -S protoc
```

### 使用tonic提供的方式编写proto编译集成

- 根目录下创建一个 `build.rs` 文件，将 `proto` 模块加入到项目的编译中来
- 这个编译会在所有的 rust 代码编译之前执行，会根据 `proto` 文件自动生成 rust 的协议代码
- 在应用中直接导入这个 `proto` 模块会实现根据服务生成的 `trait` 即可

```rs

```


## 服务测试 

- [prtotoCurl](https://github.com/qaware/protocurl)
- [grpcurl](https://github.com/fullstorydev/grpcurl)

安装上述工具即可在命令行中发送类似 `curl` 格式的内容
请求服务端提供的 `ptoto` 协议服务

```shell
grpcurl -plaintext -proto ./proto/calculator.proto \
    -d '{"a": 2, "b": 3}' \
    '[::1]:50051' calculator.CalculatorService.Add

```
---

- 添加了反射的服务之后，请求服务端就不再需要使用 `-proto ./proto/calculator.proto` 
来告知服务描述文件的位置, 这时候可以启用 grpcurl 的UI界面使用

```shell
grpcui -plaintext ’[::1]:50051‘
```


## Grpc 客户端




## Tonic 中的错误处理
