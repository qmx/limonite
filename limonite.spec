Name:		limonite
Version:	0.2.2
Release:	1%{?dist}
Summary: yet another highly-opinionated blog and static site generator

Group:	Applications/Publishing
License:	ASL 2.0
URL: http://github.com/qmx/limonite
Source0:	https://github.com/qmx/limonite/archive/limonite-%{version}-1.tar.gz#/%{name}-%{version}.tar.gz

BuildRequires:	rust-binary
BuildRequires:	cmake
BuildRequires:	openssl-devel

%description


%prep
%setup -qn %{name}-%{version}


%build
cargo build --release

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}%{_bindir}/
cp -p target/release/limonite %{buildroot}%{_bindir}/

%files
/usr/bin/limonite
%changelog
* Fri Jan 22 2016 Douglas Campos <qmx@qmx.me> 0.2.1-1
- bump up version (qmx@qmx.me)
- reversing the post list (qmx@qmx.me)

* Thu Jan 21 2016 Douglas Campos <qmx@qmx.me> 0.2.0-1
- fix release version (qmx@qmx.me)
- updating readme (qmx@qmx.me)
- bump up version to 0.2.0 (qmx@qmx.me)
- tons of cleanups (qmx@qmx.me)
- handlebars rendering complete (qmx@qmx.me)
- add walkdir (qmx@qmx.me)
- make it pretty (qmx@qmx.me)
- rendering posts with handlebars (qmx@qmx.me)
- remove most of the old layout structure (qmx@qmx.me)
- canonical blog layout (qmx@qmx.me)
- update documentation (qmx@qmx.me)
- manually implement Debug trait (qmx@qmx.me)
- generate serialization for Site (qmx@qmx.me)
- enable serde types for handlebars (qmx@qmx.me)
- bring handlebars crate in (qmx@qmx.me)
- generate serialization for Post (qmx@qmx.me)
- introduce serde (qmx@qmx.me)
- strip unused variable (qmx@qmx.me)
- split version generation on its own function (qmx@qmx.me)
- add handlebars dep (qmx@qmx.me)

* Fri Jan 15 2016 Douglas Campos <qmx@qmx.me> 0.1.0-4
- rpm build strips git info (qmx@qmx.me)

* Fri Jan 15 2016 Douglas Campos <qmx@qmx.me> 0.1.0-3
- add another missing build dep for rpm (qmx@qmx.me)

* Fri Jan 15 2016 Douglas Campos <qmx@qmx.me> 0.1.0-2
- giving a shot to the ReleaseTagger (qmx@qmx.me)
- add missing build dep for rpm (qmx@qmx.me)

* Fri Jan 15 2016 Douglas Campos <qmx@qmx.me> 0.1.0-1
- bump version to 0.1.0 (qmx@qmx.me)
- add verbose flag (qmx@qmx.me)
- better about description (qmx@qmx.me)
- another travis config goof (qmx@qmx.me)
- use a newer cmake (qmx@qmx.me)
- use --release flag for the builds (qmx@qmx.me)
- add a better cli interface (qmx@qmx.me)
- ensure GIT_VERSION is free from tito junk (qmx@qmx.me)
- add git version tag to the library (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.5-1
- fix revision before bumping (qmx@qmx.me)
- sync version (qmx@qmx.me)
- add missing empty folders (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.4-2
- bump up version on Cargo (qmx@qmx.me)
- fix license on the spec file (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.4-1
- still trying to fix the rpm build (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.3-1
- fixing bogus url, again (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.2-1
- fix url in spec (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.1-1
- new package built with tito


