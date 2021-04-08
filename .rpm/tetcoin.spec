%define debug_package %{nil}

Name: tetcoin
Summary: Implementation of a https://tetcoin.network node in Rust based on the Tetcore framework.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

Requires: systemd, shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}


%prep
%setup -q


%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%post
config_file="/etc/default/tetcoin"
getent group tetcoin >/dev/null || groupadd -r tetcoin
getent passwd tetcoin >/dev/null || \
    useradd -r -g tetcoin -d /home/tetcoin -m -s /sbin/nologin \
    -c "User account for running tetcoin as a service" tetcoin
if [ ! -e "$config_file" ]; then
    echo 'TETCOIN_CLI_ARGS=""' > /etc/default/tetcoin
fi
exit 0

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/tetcoin.service
