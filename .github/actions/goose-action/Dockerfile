FROM python:3.12.6-slim

WORKDIR /app

RUN pip install pipx
RUN pipx ensurepath && /root/.local/bin/pipx ensurepath && ln -s /root/.local/bin/* /usr/local/bin/

RUN pipx install goose-ai

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Copy profiles.yaml and plan.yaml directly
COPY profiles.yaml /root/.goose/config/profiles.yaml
COPY plan.yaml /app/plan.yaml

ENTRYPOINT ["/entrypoint.sh"]