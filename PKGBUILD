# Maintainer: Donovan Glover <https://donovan.is/>
pkgname=hyprdim
pkgver=2.2.0
pkgrel=1
pkgdesc="Automatically dim windows in Hyprland when switching between them"
arch=('x86_64')
url="https://github.com/donovanglover/hyprdim"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/donovanglover/$pkgname/archive/$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"

  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/hyprdim" "$pkgdir/usr/bin/hyprdim"

  install -Dm644 "completions/_hyprdim" "$pkgdir/usr/share/zsh/site-functions/_hyprdim"
  install -Dm644 "completions/hyprdim.bash" "$pkgdir/usr/share/bash-completion/completions/hyprdim"
  install -Dm644 "completions/hyprdim.fish" "$pkgdir/usr/share/fish/vendor_completions.d/hyprdim.fish"
  install -Dm644 "man/hyprdim.1" "$pkgdir/usr/share/man/man1/hyprdim.1"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
