FROM rust:1.74

WORKDIR /app

ENV CARGO_BUILD_TARGET_DIR=/tmp/target

RUN rustup component add rustfmt clippy rust-src \
    && cargo install --locked dprint sccache cargo-edit git-delta

RUN apt-get update && apt-get install -y --no-install-recommends sudo

ARG USERNAME=snowlhive
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd -g $USER_GID $USERNAME \
    && useradd -m -s /bin/bash -u $USER_UID -g $USER_GID -G sudo $USERNAME \
    && echo $USERNAME:$USERNAME | chpasswd \
    && echo "$USERNAME  ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

RUN apt-get install -y --no-install-recommends vim bash openssh-server curl git gettext exa openssh-server ca-certificates curl gnupg lsb-release \
    && curl -sS https://starship.rs/install.sh | sh -s -- --yes \
    && mkdir -p /etc/apt/keyrings && curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
    $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null \
    && apt-get update && apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin \
    && usermod -aG docker snowlhive \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

ENV DOCKER_COMPOSE_VERSION=2.23.3
RUN mkdir -p /usr/local/lib/docker/cli-plugins \
    && curl -SL https://github.com/docker/compose/releases/download/v${DOCKER_COMPOSE_VERSION}/docker-compose-linux-x86_64 -o /usr/local/lib/docker/cli-plugins/docker-compose \
    && chmod +x /usr/local/lib/docker/cli-plugins/docker-compose

RUN curl https://raw.githubusercontent.com/git/git/master/contrib/completion/git-prompt.sh -o /usr/share/bash-completion/completions/git-prompt \
    && curl https://raw.githubusercontent.com/git/git/master/contrib/completion/git-completion.bash -o /usr/share/bash-completion/completions/git-completion \
    && cat /usr/share/zoneinfo/Asia/Tokyo > /etc/localtime

RUN chown -R snowlhive:snowlhive /usr/local/cargo /tmp/target

USER snowlhive

RUN echo "source /usr/share/bash-completion/completions/git-completion" >> /home/snowlhive/.bashrc \
    && echo "source /usr/share/bash-completion/completions/git-prompt" >> /home/snowlhive/.bashrc \
    && echo 'eval "$(starship init bash)"' >> /home/snowlhive/.bashrc \
    && echo 'alias ls="exa"' >> /home/snowlhive/.bashrc \
    && echo '[[ -e /var/run/docker.sock ]] && sudo chmod 666 /var/run/docker.sock' >> /home/snowlhive/.bashrc
