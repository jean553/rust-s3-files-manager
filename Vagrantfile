# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "rust-s3-files-manager"

DOCKER_ENV = {
  "HOST_USER_UID" => Process.euid,
  "APP_PATH" => "/home/vagrant/rust-s3-files-manager" 
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
        "#{ENV['PWD']}/:#{DOCKER_ENV['APP_PATH']}",
      ]
    end
    app.ssh.username = "vagrant"

    app.vm.provision "local_ansible", type: "shell" do |s|
      s.env = DOCKER_ENV
      s.inline = "
        set -e
        cd $APP_PATH
        ansible-playbook provisionning/bootstrap-dev.yml
        echo 'done, you can now run `vagrant ssh`'
      "
    end
  end
end
