#!/bin/bash

####################################################################
# Change this to the parent directory of the git repo
PARENT_DIRECTORY_OF_REPO="/home/mihira/c"
export PARENT_DIRECTORY_OF_REPO="$PARENT_DIRECTORY_OF_REPO"
export PROJECT_PREFIX="ROCKETRC"
####################################################################

####################################################################
# Do not modify these env variables
export ROCKETRC_ROOT="$PARENT_DIRECTORY_OF_REPO/rocket_rc"
export ROCKETRC_SEVERS_SSH_CONFIG="$PARENT_DIRECTORY_OF_REPO/rocket_rc/deploy/ansible/ssh_config"
export ROCKETRC_ANSIBLE_HOSTS="$PARENT_DIRECTORY_OF_REPO/rocket_rc/deploy/ansible/hosts"
export ROCKETRC_CLUSTER_VARIABLES="$PARENT_DIRECTORY_OF_REPO/rocket_rc/deploy/ansible/cluster_vars/main.yml"
export ROCKETRC_ANSIBLE_CONFIG="$PARENT_DIRECTORY_OF_REPO/rocket_rc/deploy/ansible/ansible.cfg"
export ROCKETRC_ANSIBLE_SECRETS_FILE="$PARENT_DIRECTORY_OF_REPO/rocket_rc/deploy/ansible/secrets/secrets.yml"
export ROCKETRC_ANSIBLE_DIR="$PARENT_DIRECTORY_OF_REPO/rocket_rc/deploy/ansible"
export ROCKETRC_BOTO_CONFIG_FILE="$PARENT_DIRECTORY_OF_REPO/rocket_rc/deploy/node_config.json"
export ROCKETRC_BOTO_DEPLOYED_CONFIG_FILE="$PARENT_DIRECTORY_OF_REPO/rocket_rc/deploy/deployed_config.json"
####################################################################

####################################################################
# These env variables can be set
# Location of the private key to login to the nectar servers
export ROCKETRC_PRIVATE_KEY_FILE="~/.ssh/rocket_rc/rocket_rc.pem"
# The name given to the private key pair on nectar
export ROCKETRC_KEY_PAIR_NAME="rocket_rc"
# Location of AWS account credentials
export ROCKETRC_AWS_KEY_FILE="~/.ssh/personal_aws_creds"
# Location of ansible vault password file
export ROCKETRC_ANSIBLE_VAULT_PASSWORD_FILE="~/.ssh/cryptox_ansible_key"
####################################################################

