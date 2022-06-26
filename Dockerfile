FROM scratch

ARG bin

COPY ${bin} /dnsbot

VOLUME /db

CMD /dnsbot
