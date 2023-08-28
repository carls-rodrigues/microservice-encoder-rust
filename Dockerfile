FROM rust:1.71
ENV PATH="$PATH:/bin/bash" \
    BENTO4_BIN="/opt/bento4/bin" \
    PATH="$PATH:/opt/bento4/bin"

# FFMPEG
RUN apt-get -y update && apt-get -y upgrade && apt-get install -y ffmpeg bash curl make musl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/*

# Install Bento
WORKDIR /tmp/bento4
ENV BENTO4_BASE_URL="http://zebulon.bok.net/Bento4/source/" \
    BENTO4_VERSION="1.6.0-634" \
    BENTO4_CHECKSUM="5378dbb374343bc274981d6e2ef93bce0851bda1" \
    BENTO4_TARGET="" \
    BENTO4_PATH="/opt/bento4" \
    BENTO4_TYPE="SRC"
    # download and unzip bento4
RUN apt-get -y update && apt-get -y upgrade && apt-get install -y curl unzip bash gcc g++ libssl-dev cmake && \
    curl -O -s "http://zebulon.bok.net/Bento4/source/Bento4-SRC-1-6-0-634.zip"
#    sha1sum -b Bento4-${BENTO4_TYPE}-${BENTO4_VERSION}${BENTO4_TARGET}.zip | grep -o "^$BENTO4_CHECKSUM " && \
RUN mkdir -p ${BENTO4_PATH} && \
    unzip Bento4-SRC-1-6-0-634.zip -d ${BENTO4_PATH} && \
    rm -rf Bento4-SRC-1-6-0-634.zip && \
    # don't do these steps if using binary install
    cd ${BENTO4_PATH} && \
    mkdir output && cd output && \
    cmake -DCMAKE_BUILD_TYPE=Release .. && make && \
    cp -R ${BENTO4_PATH}/output ${BENTO4_PATH}/bin && \
    cp -R ${BENTO4_PATH}/Source/Python/utils ${BENTO4_PATH}/utils && \
    cp -a ${BENTO4_PATH}/Source/Python/wrappers/. ${BENTO4_PATH}/bin

WORKDIR /app

ENTRYPOINT ["tail", "-f", "/dev/null"]