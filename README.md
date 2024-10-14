# Purpose
I wrote libvirt-autoscaler to dynamically create libvirt-based kubernetes nodes.  Autoscaler handles creating these VMs, but, it will not delete nodes that are decommissioned.  This tool handles that deletion a-la cloud-controller-manager/node-manager
