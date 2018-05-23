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

COPY --from=build-deps /usr/src/app/dist /srv

ENTRYPOINT ["/usr/bin/caddy"]
CMD ["--conf", "/etc/Caddyfile", "--log", "stdout"]