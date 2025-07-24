# clean histfile

This is a very simple program.

clean home directory's `.histfile`.

make every command history no longer repeat itself.

---

这是一个非常简单的程序。

清理主目录的 `.histfile`。

使每个命令历史不再重复。

## Auto clean on shutdown

1. Create a systemd service

```shell
sudo vim /etc/systemd/system/clean_histfile.service
```

2. Paste the following content (replace the paths accordingly):

```clean_histfile.service
[Unit]
Description=Clean histfile before shutdown
DefaultDependencies=no
Before=shutdown.target

[Service]
Type=oneshot
Environment=HOME=/home/path_to_your_home
ExecStart=/path_to_your_clean_program
RemainAfterExit=true

[Install]
WantedBy=shutdown.target

```

#### Make sure to replace `/home/path_to_your_home` and `/path_to_your_clean_program` with your actual home directory path

3. Make the script executable

```shell
chmod +x /path_to_your_clean_program
```

4. Enable the service

```shell
sudo systemctl enable clean_histfile.service
```
