%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: netbrowse
Summary: A graphical frontend to avahi-browse
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPL-3.0
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/ewpratten/netbrowse

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
