# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "rust-s3-files-manager"
HOME_DIRECTORY = "/home/vagrant"
PROJECT_DIRECTORY = "#{HOME_DIRECTORY}/#{PROJECT}"

DOCKER_ENV = {
  "HOST_USER_UID" => Process.euid,
  "HOME_DIRECTORY" => "#{HOME_DIRECTORY}",
  "PROJECT_DIRECTORY" => "#{PROJECT_DIRECTORY}",
  "APP_PATH" => "#{PROJECT_DIRECTORY}/rust-s3-files-manager"
}

ENV['VAGRANT_NO_PARALLEL'] = 'yes'
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'docker'
Vagrant.configure(2) do |config|

  config.ssh.insert_key = false
  config.vm.define "dev", primary: true do |app|
    app.vm.provider "docker" do |d|
      d.image = "jean553/rust-dev-docker"
      d.name = "#{PROJECT}_dev"
      d.has_ssh = true
      d.env = DOCKER_ENV
      d.volumes =  [
        "#{ENV['PWD']}/:#{PROJECT_DIRECTORY}",
      ]
    end
    app.ssh.username = "vagrant"

    app.vm.provision "local_ansible", type: "shell" do |s|
      s.env = DOCKER_ENV
      s.inline = "
        set -e
        cd $PROJECT_DIRECTORY
        ansible-playbook provisionning/bootstrap-dev.yml
        echo 'done, you can now run `vagrant ssh`'
      "
    end
  end
end
