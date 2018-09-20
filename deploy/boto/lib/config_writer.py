import os
import json
from sets import Set

class ConfigWriter:
    def __init__(self, config):
        self.config = config
        self.nodes = self.config['nodes']
        self.proj_prefix = os.environ['PROJECT_PREFIX']

    def write_hosts_file(self):
        print '\nBuilding hosts file..'
        f = open(os.path.expanduser(os.environ[ "{}_ANSIBLE_HOSTS".format(self.proj_prefix)]), 'w')

        f.write("[all]\n")
        for name, node_info in self.nodes.items():
            f.write("{}\n".format(name))
        f.write("\n")

        ansible_host_groups = Set()
        for name, node in self.nodes.items():
            for ansible_name in node['ansible_host_groups']:
                ansible_host_groups.add(ansible_name)

        for group in ansible_host_groups:
            f.write("[{}]\n".format(group))
            for name, node_info in self.nodes.items():
                if group in node_info['ansible_host_groups']:
                    f.write("{}\n".format(name))
            f.write("\n")

        f.close()
        print 'Host file is : '
        f = open(os.path.expanduser(os.environ["{}_ANSIBLE_HOSTS".format(self.proj_prefix)]), 'r')
        print(f.read())
        f.close()

    def write_ssh_config(self):
        print '\nBuilding ssh_config..'
        f = open(os.path.expanduser(os.environ["{}_SEVERS_SSH_CONFIG".format(self.proj_prefix)]), 'w')
        for name, node_info in self.nodes.items():
            inst = node_info['inst']
            f.write("Host {}\n".format(name))
            f.write("  HostName {}\n".format(inst.ip_address))
            f.write("  User ubuntu\n")
            f.write("  IdentityFile {}\n".format(os.path.expanduser(os.environ["{}_PRIVATE_KEY_FILE".format(self.proj_prefix)])))
            f.write("\n")
        f.close
        print '\nssh_config is : '
        f = open(os.path.expanduser(os.environ["{}_SEVERS_SSH_CONFIG".format(self.proj_prefix)]), 'r')
        print f.read()
        f.close

    def write_deployed_config(self):
        print "\nBuilding deployed config"
        f = open(os.path.expanduser(os.environ["{}_BOTO_DEPLOYED_CONFIG_FILE".format(self.proj_prefix)]), 'w')
        for name, node in self.nodes.items():
            inst = node.pop('inst', None)
            node['public_dns_name'] = inst.public_dns_name
            node['private_dns_name'] = inst.private_dns_name
            node['ip_address'] = inst.ip_address
            node['private_ip_address'] = inst.private_ip_address

        f.write(json.dumps(self.config, indent=2, sort_keys=True))
        f.close()





