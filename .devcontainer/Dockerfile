FROM centos:7
WORKDIR /root/
RUN ls -l && pwd && ls -l /root/
COPY . .
RUN bash ./init.sh
ENV PATH="/root/.cargo/bin:$PATH"
