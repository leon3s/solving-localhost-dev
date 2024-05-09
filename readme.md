# Solving the Localhost Development Headache with Nanocl

Developers often face significant challenges when working on projects locally, especially when it comes to managing multiple services, dealing with CORS and cookies, and ensuring that the development environment mirrors production. However, there's a promising solution on the horizon: [Nanocl][nanocl-doc].

## Introducing Nanocl

[Nanocl][nanocl-doc] is a powerful tool designed to streamline project deployment and alleviate the pains associated with localhost development. By seamlessly integrating with your development workflow, Nanocl simplifies the process of running multiple services locally and eliminates common headaches like CORS issues and port conflicts.

## The Problem at Hand

Consider a scenario where you're working on a project consisting of various components—an API, a dashboard, and a login page. Traditionally, you would need to run each of these components on different ports, leading to cumbersome setups and potential conflicts. Additionally, issues like CORS and cookies become apparent, especially when these services aren't bound to a specific domain. Managing these challenges manually can be time-consuming and error-prone, detracting from your productivity and focus on actual development tasks.

## How Nanocl Comes to the Rescue

[Nanocl][nanocl-doc] provides a comprehensive solution to the localhost development headache. By leveraging its powerful features, developers can streamline their workflow and focus on building great software without getting bogged down by infrastructure complexities.

### Seamless Integration

[Nanocl][nanocl-doc] seamlessly integrates with your existing development environment, allowing you to deploy your project with a single command. Gone are the days of juggling multiple terminals and wrestling with configuration files—Nanocl handles it all for you.

### Domain Resolution

One of Nanocl's standout features is its ability to create domains for your local machine, bringing your development environment closer to production. With Nanocl, you can access your services using intuitive domain names like api.dev.internal, console.dev.internal, and auth.dev.internal, making testing and debugging a breeze.

### Realistic Testing Environment

By replicating the production setup locally, Nanocl enables developers to test their projects in a more realistic environment. This ensures smoother transitions from development to production and reduces the likelihood of encountering unforeseen issues down the line.

## Getting Started with Nanocl

Ready to kiss localhost headaches goodbye? Here's how to get started with Nanocl:

1.  **Installation**: Refer to the [Nanocl documentation][nanocl-doc-install] for detailed instructions on installing Nanocl on your machine.

2.  **Clone the Example Repository**: Once Nanocl is installed, clone the example repository provided to see Nanocl in action.

    ```bash
    git clone https://github.com/leon3s/solving-localhost-dev
    ```

3.  **Run the Project**: Navigate to the cloned repository and run the project using Nanocl.

    ```bash
    cd solving-localhost-dev
    nanocl state apply -fs Statefile.yml
    ```

4.  **Update `/etc/resolve.conf`**: Retrieve the IP address of the DNS server by running the following command:

    ```bash
    nanocl namespace ls
    ```

    The output will display the IP address of the DNS server, which you'll need to add to your `/etc/resolve.conf` file.

    ```bash
    NAME      CARGOES    INSTANCES    GATEWAY     CREATED AT
    system    7          7            10.1.0.1    2024-05-09 15:52:27
    global    4          4            10.2.0.1    2024-05-09 15:52:27
    ```

    Update your `/etc/resolve.conf` file with the IP address of the gateway:

    ```bash
    nameserver <global-namespace-gateway-ip>
    ```

5.  **Access Your Domains**: Once the setup is complete, you can access your services using the designated domains (api.dev.internal, console.dev.internal, auth.dev.internal). Enjoy a headache-free development experience!

## How it works

Nanocl will parse the Statefile.yml to know the current state of the services. It will then create the necessary namespaces, cargoes, and instances to run the services. The services will be available at the domains specified in the Statefile.yml.

With a combinaison of a DNS server and a reverse proxy, Nanocl will resolve the domains to the correct services.

## Key Takeaways

- **Simplified Development Workflow**: Nanocl streamlines the process of running multiple services locally, enabling developers to focus on building great software.

- **Domain Resolution**: Nanocl help you to setup intuitive domains for your services, making it easier to test and debug your projects.

- **Realistic Testing Environment**: By mirroring the production setup locally, Nanocl helps developers identify and resolve issues early on, leading to smoother deployments.

- **Seamless Integration**: Nanocl seamlessly integrates with your existing development environment, eliminating the need for manual configurations and complex setups.

## Conclusion

With Nanocl, localhost development doesn't have to be a headache anymore. By providing a seamless solution to common challenges, Nanocl empowers developers to focus on what they do best—building innovative software. Say goodbye to CORS woes, port conflicts and cookies problems, and hello to a smoother, more efficient development workflow. Give Nanocl a try today and unlock a world of possibilities for your projects. Happy coding!

[nanocl-doc]: https://next-hat.com/nanocl
[nanocl-doc-install]: https://docs.next-hat.com/manuals/nanocl/install/overview
