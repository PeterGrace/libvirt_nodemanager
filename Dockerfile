FROM docker.io/ubuntu:24.04
ARG TARGETARCH

RUN mkdir -p /opt/libvirt_nodemanager
WORKDIR /opt/libvirt_nodemanager
COPY ./tools/target_arch.sh /opt/libvirt_nodemanager
COPY ./docker/entrypoint.sh /opt/libvirt_nodemanager

RUN apt-get -y update \
  && DEBIAN_FRONTEND=noninteractive  apt-get -y install bash libvirt0 ssh \
  && useradd -u 10000 libvirt-nodemanager \
  && mkdir -p /home/libvirt-nodemanager/.ssh \
  && chown libvirt-nodemanager.libvirt-nodemanager /home/libvirt-nodemanager/.ssh \
  && chmod 700 /home/libvirt-nodemanager/.ssh

RUN --mount=type=bind,target=/context \
 cp /context/target/$(/opt/libvirt_nodemanager/target_arch.sh)/release/libvirt_nodemanager /opt/libvirt_nodemanager/libvirt_nodemanager
USER 10000
CMD ["/opt/libvirt_nodemanager/entrypoint.sh"]
EXPOSE 8443
