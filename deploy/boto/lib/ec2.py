import boto
import time
import os
import ConfigParser
import json

class Ec2:
    def __init__(self, conn):
        self.conn = conn
        self.proj_prefix = os.environ['PROJECT_PREFIX']
        self.config = self._get_config()

    def _get_config(self):
        proj_prefix = os.environ['PROJECT_PREFIX']
        f = open(os.environ["{}_BOTO_CONFIG_FILE".format(proj_prefix)], 'r')
        ret = json.loads(f.read())
        f.close()
        return ret

    def _check_has_tags(self, instance, tag_key):
        return tag_key in instance.tags

    def _expected_node_names(self):
        nodes = dict(self.config['nodes'])
        node_names = [v['tags']['name'] for k,v in nodes.items()]
        return node_names

    def current_non_term_tagged_instances(self):
        current_instances = self.conn.get_only_instances()
        ret_insts = []
        for inst in current_instances:
            inst_tags = inst.tags
            if inst.state_code !=48 and \
                'name' in inst_tags and \
                inst_tags['name'] in self._expected_node_names():
                ret_insts.append(inst)
        return ret_insts

    def get_nodes_hydrated(self):
        current_instances = self.current_non_term_tagged_instances()
        new_config = dict(self._get_config())
        for inst in current_instances:
            name = inst.tags['name']
            new_config['nodes'][name]['inst'] = inst
        return new_config

    def missing_nodes(self):
        current_instances = self.conn.get_only_instances()
        expected_node_names = self._expected_node_names()
        nodes = self.config['nodes']
        for inst in self.current_non_term_tagged_instances():
            inst_tags = inst.tags
            expected_node_names.remove(inst_tags['name'])
        return {x:nodes[x] for x in expected_node_names}

        return list(filter(lambda x: x.state_code != 48 and self._check_has_tags(x, self.config.get('node', 'TAG')), current_instances))

    def start_instances(self, missing_nodes_dict):
        to_create_nodes = dict(missing_nodes_dict)
        for name,node_json in to_create_nodes.items():
            print "Creating {} ...".format(name)
            node_json['reservation'] = self.conn.run_instances(
                node_json['ami'],
                min_count=1,
                max_count=1,
                key_name=os.environ[ "{}_KEY_PAIR_NAME".format(self.proj_prefix) ],
                instance_type=node_json['type'],
                security_groups=node_json['security_groups']
            )
            node_json['inst'] = node_json['reservation'].instances[0]
        self.wait_for_insts_to_start(to_create_nodes)
        self._attach_volumes(to_create_nodes)
        self._attach_tags_for_insts(to_create_nodes)
        return to_create_nodes

    def _attach_volumes(self, created_nodes):
        for name, node_info in created_nodes.items():
            if node_info['attach_volume']:
                vol_name = node_info['volume']
                mnt_pnt = node_info['volume_mount_point']
                print "Mounting {} to {} at {}".format(vol_name, name, mnt_pnt)
                self.conn.attach_volume(vol_name, node_info['inst'].id, mnt_pnt)

    def print_non_term_tagged_inst_info(self):
        for inst in self.current_non_term_tagged_instances():
            print("id : {}".format(inst.id))
            print("public_ip: {}".format(inst.ip_address))
            print("private_ip: {}".format(inst.private_ip_address))
            print("groups: {}".format(inst.groups))
            print("")

    def wait_non_term_tagged_insts_started(self):
        status_codes = []
        while True:
            non_term_insts = self.current_non_term_tagged_instances()
            if len(non_term_insts) == 0:
                break
            if len(status_codes) > 0 and all([ i == 16 for i in status_codes ]):
                break
            status_codes = []
            print "\nWaiting for all instances to startup ..."
            for reservation in non_term_insts:
                print "Reservation {} status is : {}".format(reservation.id, reservation.state_code)
                status_codes.append(reservation.state_code)
                print "------------------"
            time.sleep(2)

    def wait_for_insts_to_start(self, create_nodes_dict):
        insts = [x['inst'] for k,x in create_nodes_dict.items()]
        status_codes = []

        while True:
            print "\nWaiting for all instances to startup ..."
            for inst in insts:
                inst.update()
                print "Instance {} status is : {}".format(inst.id, inst.state_code)
                status_codes.append(inst.state_code)
            if all([ i == 16 for i in status_codes ]):
                break
            status_codes = []
            time.sleep(2)

    def _attach_tags_for_insts(self, created_nodes):
        for name, node_obj in created_nodes.items():
            for tag_key, tag_val in node_obj['tags'].items():
                node_obj['inst'].add_tag(tag_key, tag_val)
