#!/usr/bin/python
import boto
import os
import json
import time

from lib.con import Con
from lib.ec2 import Ec2
from lib.config_writer import ConfigWriter

from boto.ec2.connection import EC2Connection
from boto.ec2.regioninfo import *

def pp(_json):
    print json.dumps(_json, indent=2, sort_keys=True)

class InstanceLauncer:
    def __init__(self):
        proj_prefix = os.environ['PROJECT_PREFIX']
        f = open(os.environ["{}_BOTO_CONFIG_FILE".format(proj_prefix)], 'r')
        self.config = json.loads(f.read())

        self.conn = Con().conn
        self.ec2 = Ec2(self.conn)

    def _launch_instances(self):
        missing_nodes = self.ec2.missing_nodes()
        self.created_nodes = self.ec2.start_instances(missing_nodes)

        print '\nAll instances are running\n'

    def _write_config(self):
        config_writer = ConfigWriter(self.ec2.get_nodes_hydrated())
        config_writer.write_hosts_file()
        config_writer.write_ssh_config()
        config_writer.write_deployed_config()

    def run(self):

        self._launch_instances()
        self.ec2.print_non_term_tagged_inst_info()
        self._write_config()

InstanceLauncer().run()
