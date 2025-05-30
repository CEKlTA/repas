# Maintainer: cekita <ceka.dora@ejemplo.com>
pkgname=repas
pkgver=0.1.0
pkgrel=1
pkgdesc="A blazingly fast CLI tool for managing files and folders with symbolic links"
arch=('x86_64')
url="https://github.com/CEKlTA/repas"
license=('MIT')
depends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/CEKlTA/repas/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('e36dabc2d8c3efd9496d6ebff51daa40b773f416929070f3f5a542310521515e')

build() {
  cd "$srcdir/repas-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/repas-$pkgver"
  install -Dm755 "target/release/repas" "$pkgdir/usr/bin/repas"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
