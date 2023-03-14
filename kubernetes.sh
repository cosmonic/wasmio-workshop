#!/usr/bin/env bash

set -euo pipefail

CREDS_FILE="${HOME}/.cosmo/user.creds"
REPLICA_COUNT=2

if ! command -v helm &> /dev/null; then
    echo "Helm is not installed, unable to continue"
    exit 1
fi

if [[ ! -f "$CREDS_FILE" ]]; then
    echo "Not currently logged in, running 'cosmo login'"
    cosmo login
fi

# Get the environment variables we need
eval "$(cosmo up --show-env)"

echo -n "${WASMCLOUD_CLUSTER_SEED}" > clusterseed.nk
chmod 600 clusterseed.nk

echo "Generating Helm values file in values.yaml"
cat <<EOF > values.yaml
replicaCount: ${REPLICA_COUNT}
wasmcloud:
  config:
    jetstreamDomain: cosmonic
    latticePrefix: ${WASMCLOUD_LATTICE_PREFIX}
    clusterIssuers: ${WASMCLOUD_CLUSTER_ISSUERS}
    controlTopicPrefix: cosmo.ctl
    hostLabels:
      stargate: "true"
  enableApplierSupport: true
  customLabels:
    wasmcloud.dev/route-to: "true"
nats:
  jetstreamDomain: stargate
  leafnode:
    enabled: true
    clusterURL: "tls://connect.cosmonic.sh"
EOF

if ! helm repo list | grep wasmcloud &> /dev/null; then 
    echo "Adding wasmcloud helm repository"
    helm repo add wasmcloud https://wasmcloud.github.io/wasmcloud-otp/
fi

echo "Updating wasmcloud helm repository"
helm repo update wasmcloud 1>/dev/null

echo "Installing Helm chart"
helm install --wait -f values.yaml --set-file "nats.leafnode.credentials=${HOME}/.cosmo/user.creds" --set-file wasmcloud.config.clusterSeed=clusterseed.nk wasmcloud-workshop wasmcloud/wasmcloud-host

# Give time for things to put their pants on. This could be improved by checking that the number of hosts returned from `wash ctl get hosts` increases by 2
sleep 3

echo "Starting httpservers"
for ((i=1; i <= REPLICA_COUNT; i++))
do
    # If we don't skip wait, it sits forever
    wash ctl start provider -x "${WASMCLOUD_LATTICE_PREFIX}" -r tls://connect.cosmonic.sh --ctl-credsfile "${HOME}/.cosmo/user.creds" -c "kubernetes=true" wasmcloud.azurecr.io/httpserver:0.17.0 --skip-wait 1>/dev/null || echo "WARN: Unable to start provider. Run the following command to retry when the script exits: wash ctl start provider -x ${WASMCLOUD_LATTICE_PREFIX} -r tls://connect.cosmonic.sh --ctl-credsfile ${HOME}/.cosmo/user.creds -c kubernetes=true wasmcloud.azurecr.io/httpserver:0.17.0 --skip-wait"
    echo "Waiting for provider ${i}/${REPLICA_COUNT} to start"
    # If we try too fast, it doesn't work most of the time
    sleep 5
done

echo "Starting service applier providers and actor"
wash ctl start provider -x "${WASMCLOUD_LATTICE_PREFIX}" -r tls://connect.cosmonic.sh --ctl-credsfile "${HOME}/.cosmo/user.creds" -c "kubernetes=true" wasmcloud.azurecr.io/applier:0.3.0 --skip-wait 1>/dev/null
wash ctl start provider -x "${WASMCLOUD_LATTICE_PREFIX}" -r tls://connect.cosmonic.sh --ctl-credsfile "${HOME}/.cosmo/user.creds" -c "kubernetes=true" wasmcloud.azurecr.io/nats_messaging:0.16.3 --skip-wait 1>/dev/null
wash ctl start actor -x "${WASMCLOUD_LATTICE_PREFIX}" -r tls://connect.cosmonic.sh --ctl-credsfile "${HOME}/.cosmo/user.creds" -c "kubernetes=true" wasmcloud.azurecr.io/service_applier:0.3.0 --skip-wait 1>/dev/null

echo "Linking applier"
wash ctl link put -x "${WASMCLOUD_LATTICE_PREFIX}" -r tls://connect.cosmonic.sh --ctl-credsfile "${HOME}/.cosmo/user.creds" MCF7GSDHIC6DMOXAJKD747RIRS7I54H7OIDEWGA5JMZJF3WNH6KNIKO3 VADNMSIML2XGO2X4TPIONTIC55R2UUQGPPDZPAVSC2QD7E76CR77SPW7 wasmcloud:messaging "SUBSCRIPTION=wasmbus.evt.${WASMCLOUD_LATTICE_PREFIX}" 1>/dev/null
wash ctl link put -x "${WASMCLOUD_LATTICE_PREFIX}" -r tls://connect.cosmonic.sh --ctl-credsfile "${HOME}/.cosmo/user.creds" MCF7GSDHIC6DMOXAJKD747RIRS7I54H7OIDEWGA5JMZJF3WNH6KNIKO3 VDW26HWKJZMKRNIRABM3BEF7BP23XECL7CO4JAAUZ5YEB6UGQFVK7ANR cosmonic:kubernetes_applier 1>/dev/null
