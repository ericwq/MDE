# pi.dev 安装步骤

## 安装 Nono

- 访问 [nono.sh](https://nono.sh/docs/cli/getting_started/installation) 
- 使用 brew 进行安装 `brew install nono`
- 将 [pi-dev.json](./pi-dev.json) 添加至目录 `~/.config/nono/profiles/` (保持文件名不变)
- 运行 `nono profile list` 确保配置文件没有错误
- 请参考下图

![Nono profile list](./img/nono-profile-list.png)

## 启动 LM Studio

- 在 Developer 页签，启动并加载模型：`mlx-community/qwen3.6-35b-a3b`
- 设置模型的上下文大小：262144
- 检查 LM Studio 服务器运行在 1234 端口：`curl http://127.0.0.1:1234/v1/models` 能返回模型列表 → 正常
- 下面的配置文件中使用模型：`mlx-community/qwen3.6-35b-a3b`
- 请参考下图

![LM Studios developer panel](./img/lm-studios.png)

## 安装 pi.dev

- 访问 [pi.dev](https://pi.dev/)
- 运行安装程序 `curl -fsSL https://pi.dev/install.sh | sh`
- 进入授权的工作目录 `cd $HOME/dev/projects` (否则 nono 会询问你是否授权读写当前目录)
- 运行 pi.dev `nono run -v --profile pi-dev pi` 同时打印所有授权内容 (此时你可以检查授权了对哪些资源的访问)
- 将 [pi-dev-models.json](./pi-dev-models.json) 拷贝至 `~/.pi/agent/models.json` (文件名更改为models.json，添加上一节中启动的模型)
- 运行 pi.dev `nono run --profile pi-dev pi`
- 请参考下图

![pi.dev start](./img/nono-pi-dev.png)

