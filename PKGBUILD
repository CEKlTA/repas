# Maintainer: cekita <ceka.dora@gmail.com>
pkgname=repas
pkgver=0.2.1
pkgrel=1
pkgdesc="A blazingly fast CLI tool for managing files and folders with symbolic links"
arch=('x86_64')
url="https://github.com/CEKlTA/repas"
license=('MIT')
makedepends=('rust' 'cargo')
depends=()
provides=('repas')
conflicts=()
source=("git+https://github.com/CEKlTA/repas.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/repas"
  git describe --tags --long | sed 's/^v//;s/-/.r/;s/-/./'
}

build() {
  cd "$srcdir/repas"
  cargo build --release --locked
}

package() {
  install -Dm755 "$srcdir/repas/target/release/repas" "$pkgdir/usr/bin/repas"
}
