# WhisperingOaks: GnuPG Wrapper for Text Editors

WhisperingOaks is a Linux command-line tool that acts as a GnuPG encryption
wrapper for text editors. It provides a secure and convenient way to encrypt
text files while using the text editor of your choice, utilizing the power of
GnuPG encryption.

WhisperingOaks will utilize your default editor specified in environment
variable EDITOR by default. If you want WhisperingOaks to utilize another editor
by default, add; `export WHISPERING_OAKS="<editor-of-choice>"`

WhisperingOaksa are dependent on following:
- GnuPG.
- GnuPG Public Key for encryption.
- Pinentry (System needs to allow for pinentry-mode loopback).
- Editor such as Helix, Neovim, Nano, etc. (Can work with some GUI editors).
