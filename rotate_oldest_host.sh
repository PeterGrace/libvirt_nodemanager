#!/bin/bash
 
oldest=$(kubectl get nodes -o json | jq -rc '.items[]| select(.metadata.name | test("libvirt"))|.metadata.creationTimestamp |= fromdateiso8601|[.metadata.creationTimestamp,.metadata.name]|@csv'|sort -n|head -1|cut -d, -f2)
echo $oldest |sed s/\"//g

