FROM python:3.7.4-stretch
ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
RUN sed -i "s@http://deb.debian.org@http://mirrors.aliyun.com@g" /etc/apt/sources.list
RUN sed -i "s@http://security.debian.org@http://mirrors.aliyun.com@g" /etc/apt/sources.list && rm -rf /var/lib/apt/lists/* && apt-get update
RUN apt-get install -y gcc g++ openssl tar wget
RUN pip install --upgrade cmake -i https://pypi.tuna.tsinghua.edu.cn/simple
RUN curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | bash -s -- -y
RUN echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.bashrc
RUN mkdir -p $HOME/.cargo
COPY ./fc_rust_env/config $HOME/.cargo/config
CMD ["bash"]