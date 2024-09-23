# Dockerfile
FROM ruby:latest
COPY hello.rb /app/hello.rb
WORKDIR /app
CMD ["ruby", "/app/hello.rb"]
