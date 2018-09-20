import boto
import os
import json
import time
import sys, paramiko
from lib.con import Con
from lib.ec2 import Ec2

from boto.ec2.connection import EC2Connection
from boto.ec2.regioninfo import *

conn = Con().conn
ec2 = Ec2(conn)
instances = ec2.current_non_term_tagged_instances()

ssh_config = paramiko.SSHConfig()
proj_prefix = os.environ['PROJECT_PREFIX']
user_config_file = os.path.expanduser(os.environ[ "{}_SEVERS_SSH_CONFIG".format(proj_prefix) ])
if os.path.exists(user_config_file):
    with open(user_config_file) as f:
        ssh_config.parse(f)

host_names = ssh_config.get_hostnames()
host_by_ip = {}
for host_name in host_names:
    host  = ssh_config.lookup(host_name)
    host_by_ip[host['hostname']] = host

if len(instances) == 0:
    print "No instances available."
    exit(0)

print "\nInstances are : {}".format(instances)


print "\nWaiting for ssh availability"
avail = []
while True:
    if len(avail) > 0 and all(avail):
        break
    print "Waiting ... {}".format(avail)
    time.sleep(1)
    avail = []
    for inst in instances:
        try:
            client = paramiko.SSHClient()
            client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            client.connect(inst.ip_address,
                           port=22,
                           username=host_by_ip[str(inst.ip_address)]['user'],
                           key_filename=host_by_ip[str(inst.ip_address)]['identityfile'][0])
            client.close()
            avail.append(True)
        except Exception as e:
            print "Exception! : {}".format(e)
            client.close()
            avail.append(False)
print"\nsshd running on all instances\n"
