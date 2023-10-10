packages = [
  "jq",
  "vim.org/vim",
  "aws"
]

envs {
  EDITOR = "vim"
}

file ".screenrc" {
  source = "dotfiles/.screenrc"
}

package_manager = "pkgx"