# Solving the localhost development headache

Hello there, it's been quite a while now that im working on a project that aim to help developers to solve the deployment headache, and i think i've found a solution that can help you to solve the localhost development as well.

Let's take as example a simple api, a dashboard and a login page, you can run all of them in your localhost, but you will have to run them in different ports, and you will have to deal with CORS, and cookies issues if you don't bind them to a domain, and you will have to deal with the fact that you will have to run all of them in different terminals.
Where docker compose can solve this problem, it doesn't come with a solution to create domain for your local machine.
This is where [Nanocl](https://next-hat.com/nanocl) comes in, it's a tool that have been designed to help you to deploy your project in a single command, and it can help you to solve the localhost development headache as well.

This repo is a simple example of how you can use Nanocl to solve the localhost development headache, it's a simple project that have a simple api, a dashboard and a login page, and you can run them all in your localhost in a single command.

## How to run the project

First you will have to install Nanocl

Refer to the [Nanocl documentation](https://docs.next-hat.com/manuals/nanocl/install/overview) to set it up on your machine.

Then you will have to clone this repo

```bash
git clone https://github.com/leon3s/solving-localhost-dev
```

Then you will have to run the project

```bash
cd solving-localhost-dev
nanocl state apply -f
```

In this example have created a dns server that will resolve the following domains to your localhost.
To be able to use this dns server you will have to update your `/etc/resolve.conf` to add the ip address of the dns server.

You can retrieve the ip address of the dns server by running the following command

```bash
nanocl namespace ls
```

You will get an output like this

```bash
NAME      CARGOES    INSTANCES    GATEWAY     CREATED AT
system    7          7            10.1.0.1    2024-05-09 15:52:27
global    4          4            10.2.0.1    2024-05-09 15:52:27
```

The line that you are interested in is the one that have the name `global`, you will have to add the ip address of the gateway to your `/etc/resolve.conf` file.

```bash
nameserver 10.2.0.1
```

Once you have updated your `/etc/resolve.conf` file you can now access the following domains

- api.dev.internal
- console.dev.internal
- auth.dev.internal

The project is running in dev mode so any change that you make to the code will be reflected in the running instance.

## How it works

Nanocl comes with a dns server and a proxy that will resolve the domain to the ip address of the gateway, and it will forward the request to the right instance.

This way your development setup is closer to the production setup, and you can test your project in a more realistic environment.

## Conclusion

I hope that this example will help you to solve the localhost development headache, and that you will find Nanocl useful for your project. If you have any question or suggestion feel free to open an issue or a discussion in the repo.

Happy coding!
