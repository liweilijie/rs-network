# network

**rust** 与 **network** 相关的实现。

需要考虑的一些实际的问题：

- 如何才能高并发: 使用 tokio 类型无栈协程的方式来处理每一个请求。
- 如何共享数据： 使用 channel 对全局的一些数据进行共享。
- 实现Http 的 Json 服务器： Rocket Http 来实现。
- protobuf 自定义协议：使用 protobuf 构建协议消息的时候需要注意，因为 protobuf 生成的是不定长消息，所以你需要在客户端和服务器之间约定好，如何界定一个消息帧（frame）。
- gRPC使用了 protobuf,它有自已的格式来界定消息长度。
- 自定义协议：使用自定义协议来实现。


## 自定义协议
 使用 tokio 提供的 LengthDelimitedCodec+Framed 来实现。