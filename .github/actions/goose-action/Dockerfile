FROM python:3.12.6-slim

WORKDIR /app

RUN pip install pipx
RUN pipx ensurepath

# Ensure paths
ENV PATH=$PATH:/root/.local/bin

RUN pipx install goose-ai

# Install ripgrep, curl, git, jq, and wget
RUN apt-get update && \
    apt-get install -y ripgrep \
                          curl \
                          git \
                          jq \
                          wget && \
    rm -rf /var/lib/apt/lists/*

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

COPY plan.yaml /app/plan.yaml

ENTRYPOINT ["/entrypoint.sh"]