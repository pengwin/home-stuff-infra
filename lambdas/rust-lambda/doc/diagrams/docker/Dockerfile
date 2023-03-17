FROM python:3-alpine

RUN apk update && apk add --no-cache graphviz gcc libc-dev g++ imagemagick inkscape ttf-opensans curl fontconfig xdg-utils

RUN curl -O https://noto-website.storage.googleapis.com/pkgs/NotoSansCJKjp-hinted.zip
RUN mkdir -p /usr/share/fonts/NotoSansCJKjp
RUN unzip NotoSansCJKjp-hinted.zip -d /usr/share/fonts/NotoSansCJKjp/
RUN rm NotoSansCJKjp-hinted.zip
RUN fc-cache -fv

RUN pip install diagrams