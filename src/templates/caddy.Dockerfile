# Builder
FROM abiosoft/caddy:builder as builder

ARG version="0.10.10"
ARG plugins="git"

RUN VERSION=${version} PLUGINS=${plugins} /bin/sh /usr/bin/builder.sh

# Build caddy
FROM alpine:3.6

LABEL caddy_version="0.10.10"

RUN apk add --no-cache openssh-client git

# install caddy
COPY --from=builder /install/caddy /usr/bin/caddy

# validate install
RUN /usr/bin/caddy -version
RUN /usr/bin/caddy -plugins

EXPOSE 80 443
VOLUME /root/.caddy
WORKDIR /srv

RUN echo "shop.pwa.m2:443 { proxy http://entry { transparent }  tls self_signed }" > /etc/Caddyfile

ENTRYPOINT ["/usr/bin/caddy"]
CMD ["--conf", "/etc/Caddyfile", "--log", "stdout"]