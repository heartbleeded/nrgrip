# This Makefile is meant to generate the documentation.
# To build the project, use Cargo directly!

RONN = ronn
MANPAGE = nrgrip.1

$(MANPAGE): README.md
	$(RONN) <$< >$@
