# Use Red Hat Universal Base Image 7 as the base image
#FROM registry.access.redhat.com/ubi7/ubi:latest

FROM centos:7


# Update packages and install development tools
RUN yum -y update && \
    yum -y install gcc \
    yum -y install https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm && \
    yum -y install rpm-build rpmdevtools wget curl

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Create the RPM build directories
RUN rpmdev-setuptree

# Set working directory to the RPM build directory
WORKDIR /root/rpmbuild

# Copy Rust source code tarball and spec file
COPY server_mig-0.1.1.tar.gz /root/rpmbuild/SOURCES/
COPY server_mig.spec /root/rpmbuild/SPECS/

# Optionally, build RPM immediately when container starts (you can comment this out if not needed)
# ENTRYPOINT ["rpmbuild", "-ba", "SPECS/my-spec-file.spec"]

# Or keep it interactive so you can run build commands manually
CMD ["/bin/bash"]

