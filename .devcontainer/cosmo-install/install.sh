#!/bin/bash
set -e

bash -c "$(curl -fsSL https://cosmonic.sh/install.sh)"

echo "export PATH=\"/home/vscode/.cosmo/bin:\${PATH}\"" >> "/home/vscode/.bashrc"

mkdir -p /home/vscode/.cosmo/bin
mv /root/.cosmo/bin/cosmo /home/vscode/.cosmo/bin
sudo chown -R vscode ~/.cosmo