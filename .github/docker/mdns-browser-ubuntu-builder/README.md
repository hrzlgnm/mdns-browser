# mDNS-Browser Ubuntu Builder Docker Image

## Updating the Docker image

> [!NOTE]
> This is usually not needed to do manually, as the Docker image is automatically built and pushed
> by the GitHub Actions workflow defined in the `.github/workflows/docker-build.yml` file.

Pick a version number for the new Docker image (e.g. `v2`), then run the
following commands:

    $ docker build --tag ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:VERSION_NUMBER_HERE .github/docker/mdns-browser-ubuntu-builder/
    $ docker login ghcr.io -u YOUR_GITHUB_USER_NAME_HERE
    $ docker push ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:VERSION_NUMBER_HERE

Then, change the container tag in each workflow file in the .github/workflows/
directory to refer to your new version.
