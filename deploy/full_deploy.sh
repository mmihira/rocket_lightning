#!/bin/bash
rm -rf ~/.ansible/*

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
root="$PROJECT_PREFIX""_ROOT"

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
        "pg" \
        "rc_signal" \
         "rc_server" \
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

containsElement "pg" "${to_run[@]}"
if [ $? -eq 0 ]; then
  echo "Installing postgres"
  echo "--------------------------------------------------------------"
  ./ansible/roles/postgres/scripts/tar_context.sh
  ( cd "${!root}"; cargo make save_docker_build_diesel_cli )
  ansible-playbook -s ./ansible/playbooks/postgres.yml \
                    -i "${!ansible_hosts}" \
                    --ssh-common-args="-F ${!ssh_config}" \
                    --vault-password-file "${!vault_key_file}" \
                    --extra-vars "@${!vault_file}" \
                    --extra-vars "@${!deployed_node_config}"
fi

containsElement "rc_signal" "${to_run[@]}"
if [ $? -eq 0 ]; then
  echo "Installing rc_signal"
  echo "--------------------------------------------------------------"
  ( cd "${!root}"; cargo make save_docker_build_rc_signal )
  ansible-playbook -s ./ansible/playbooks/rc_signal.yml \
                    -i "${!ansible_hosts}" \
                    --ssh-common-args="-F ${!ssh_config}" \
                    --vault-password-file "${!vault_key_file}" \
                    --extra-vars "@${!vault_file}" \
                    --extra-vars "@${!deployed_node_config}"
fi

containsElement "rc_server" "${to_run[@]}"
if [ $? -eq 0 ]; then
  echo "Installing rc server"
  echo "--------------------------------------------------------------"
  ( cd "${!root}"; cargo make save_docker_build_rc_server )
  ansible-playbook -s ./ansible/playbooks/rc_server.yml \
                    -i "${!ansible_hosts}" \
                    --ssh-common-args="-F ${!ssh_config}" \
                    --vault-password-file "${!vault_key_file}" \
                    --extra-vars "@${!vault_file}" \
                    --extra-vars "@${!deployed_node_config}"
fi
