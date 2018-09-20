#!/bin/bash

containsElement () {
  local e match="$1"
  shift
  for e; do [[ "$e" == "$match" ]] && return 0; done
  return 1
}

source config.sh

ansible_hosts="$PROJECT_PREFIX""_ANSIBLE_HOSTS"
ssh_config="$PROJECT_PREFIX""_SEVERS_SSH_CONFIG"
vault_key_file="$PROJECT_PREFIX""_ANSIBLE_VAULT_PASSWORD_FILE"
vault_file="$PROJECT_PREFIX""_ANSIBLE_SECRETS_FILE"
ansible_config="$PROJECT_PREFIX""_ANSIBLE_CONFIG"
export ANSIBLE_CONFIG=${!ansible_config}
node_config="$PROJECT_PREFIX""_BOTO_CONFIG_FILE"
deployed_node_config="$PROJECT_PREFIX""_BOTO_DEPLOYED_CONFIG_FILE"

# Create the instances
python  ./boto/launch_instances.py

# Wait for ssh connectivity
python ./boto/wait_for_sshd.py

# Do initial installs
python ./boto/init_install.py

# Delete temporary pyc files
rm boto/lib/*.pyc
rm boto/*.pyc
rm ansible/*.retry
rm ansible/playbooks/*.retry

to_run=( \
        "docker" \
        "global_env" \
        )

containsElement "docker" "${to_run[@]}"
if [ $? -eq 0 ]; then
  echo "Installing docker"
  echo "--------------------------------------------------------------"
  ansible-playbook -s ./ansible/playbooks/docker.yml \
                    -i "${!ansible_hosts}" \
                    --ssh-common-args="-F ${!ssh_config}" \
                    --vault-password-file "${!vault_key_file}" \
                    --extra-vars "@${!vault_file}"
fi

containsElement "global_env" "${to_run[@]}"
if [ $? -eq 0 ]; then
  echo "Installing global env"
  echo "--------------------------------------------------------------"
  ansible-playbook -s ./ansible/playbooks/global_env.yml \
                    -i "${!ansible_hosts}" \
                    --ssh-common-args="-F ${!ssh_config}" \
                    --vault-password-file "${!vault_key_file}" \
                    --extra-vars "@${!vault_file}" \
                    --extra-vars "@${!deployed_node_config}"
fi
