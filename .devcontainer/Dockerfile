FROM almalinux:7
WORKDIR /root/
COPY . .
RUN bash ./init.sh
ENV PATH="/root/.cargo/bin:$PATH"
