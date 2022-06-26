FROM debian:bullseye-slim

ARG bin=dnsbot

COPY ${bin} /dnsbot

VOLUME /db

CMD /dnsbot
